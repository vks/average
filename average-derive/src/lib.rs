use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

/// Derive the [`average::Merge`] trait for a struct.
///
/// Each field must implement [`average::Merge`]. The generated implementation
/// calls `merge` on every field in declaration order.
///
/// # Example
///
/// ```rust,ignore
/// use average::{Mean, Max, Merge};
///
/// #[derive(Default, Merge)]
/// struct Stats {
///     mean: Mean,
///     max: Max,
/// }
///
/// let mut a = Stats::default();
/// let mut b = Stats::default();
/// a.merge(&b);
/// ```
#[proc_macro_derive(Merge)]
pub fn derive_merge(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_types: Vec<_>;
    let merge_fields = match &input.data {
        Data::Struct(s) => {
            field_types = s.fields.iter().map(|f| &f.ty).collect();
            match &s.fields {
                Fields::Named(fields) => {
                    let calls = fields.named.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { self.#ident.merge(&other.#ident); }
                    });
                    quote! { #(#calls)* }
                }
                Fields::Unnamed(fields) => {
                    let calls = (0..fields.unnamed.len()).map(|i| {
                        let idx = syn::Index::from(i);
                        quote! { self.#idx.merge(&other.#idx); }
                    });
                    quote! { #(#calls)* }
                }
                Fields::Unit => quote! {},
            }
        }
        Data::Enum(_) | Data::Union(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "`#[derive(Merge)]` is only supported on structs",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut generics = input.generics.clone();
    {
        let where_clause = generics.make_where_clause();
        for ty in &field_types {
            where_clause
                .predicates
                .push(syn::parse_quote!(#ty: ::average::__private::Merge));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::average::__private::Merge for #name #ty_generics #where_clause {
            fn merge(&mut self, other: &Self) {
                #merge_fields
            }
        }
    }
    .into()
}

/// Concatenate several iterative estimators into one struct.
///
/// This is a proc-macro alternative to the [`concatenate!`] declarative macro.
/// Annotate a struct whose fields are estimator types (implementing
/// [`Estimate`]). The macro generates:
///
/// - `new()` and `Default`
/// - `add(&mut self, x: f64)` forwarding to every field
/// - Accessor methods for each statistic (via `#[statistics(...)]`)
/// - `FromIterator<f64>` and `FromIterator<&f64>`
///
/// To also generate a `Merge` implementation, add `#[derive(Merge)]` to the
/// struct.
///
/// # Field attributes
///
/// - `#[statistics(method1, method2, ...)]` — expose accessor methods that
///   delegate to the field. Each `method` must be a method on the field type
///   that takes `&self` and returns `f64`.
/// - If no `#[statistics(...)]` is given, a single accessor named after the
///   field is generated (i.e., `fn field_name(&self) -> f64` calls
///   `self.field_name.field_name()`).
///
/// # Example
///
/// ```rust,ignore
/// use average::{concatenate, Min, Max, Estimate};
///
/// #[concatenate]
/// struct MinMax {
///     min: Min,
///     max: Max,
/// }
///
/// let s: MinMax = (1..6).map(f64::from).collect();
/// assert_eq!(s.min(), 1.0);
/// assert_eq!(s.max(), 5.0);
/// ```
///
/// For estimators that expose multiple statistics, use `#[statistics(...)]`:
///
/// ```rust,ignore
/// use average::{concatenate, Max, Variance, Estimate};
///
/// #[concatenate]
/// struct Estimator {
///     #[statistics(mean, sample_variance)]
///     variance: Variance,
///     #[statistics(max)]
///     max: Max,
/// }
///
/// let e: Estimator = (1..6).map(f64::from).collect();
/// assert_eq!(e.mean(), 3.0);
/// assert_eq!(e.sample_variance(), 2.5);
/// assert_eq!(e.max(), 5.0);
/// ```
#[proc_macro_attribute]
pub fn concatenate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);
    match concatenate_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn concatenate_impl(input: syn::ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let vis = &input.vis;

    let fields = match &input.fields {
        Fields::Named(f) => &f.named,
        _ => {
            return Err(syn::Error::new_spanned(
                name,
                "`#[concatenate]` requires a struct with named fields",
            ));
        }
    };

    // Collect field info: (ident, type, statistics methods)
    let mut field_idents = Vec::new();
    let mut field_types = Vec::new();
    let mut all_accessors: Vec<(Ident, Ident)> = Vec::new(); // (method_name, field_ident)

    for field in fields {
        let ident = field.ident.as_ref().unwrap().clone();
        let ty = &field.ty;
        field_types.push(ty.clone());

        let statistics = parse_statistics_attr(field)?;
        if statistics.is_empty() {
            // Default: accessor named after the field, delegating to same-named method
            all_accessors.push((ident.clone(), ident.clone()));
        } else {
            for stat in statistics {
                all_accessors.push((stat, ident.clone()));
            }
        }

        field_idents.push(ident);
    }

    // Generate accessor methods
    let accessor_methods = all_accessors.iter().map(|(method, field)| {
        quote! {
            #[inline]
            #vis fn #method(&self) -> f64 {
                self.#field.#method()
            }
        }
    });

    // Generate add calls
    let add_calls = field_idents.iter().map(|ident| {
        quote! { ::average::__private::Estimate::add(&mut self.#ident, x); }
    });

    // Generate default field init
    let default_fields = field_idents.iter().map(|ident| {
        quote! { #ident: ::core::default::Default::default() }
    });

    // Re-emit the struct definition as-is (stripping our attributes)
    let struct_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let field_vis = &f.vis;
        // Preserve non-statistics attributes
        let attrs: Vec<_> = f.attrs.iter().filter(|a| !a.path().is_ident("statistics")).collect();
        quote! { #(#attrs)* #field_vis #ident: #ty }
    });

    let other_attrs = &input.attrs;

    Ok(quote! {
        #(#other_attrs)*
        #vis struct #name {
            #(#struct_fields,)*
        }

        impl #name {
            #[inline]
            #vis fn new() -> #name {
                #name {
                    #(#default_fields,)*
                }
            }

            #[inline]
            #vis fn add(&mut self, x: f64) {
                #(#add_calls)*
            }

            #(#accessor_methods)*
        }

        impl ::core::default::Default for #name {
            fn default() -> #name {
                #name::new()
            }
        }

        impl ::core::iter::FromIterator<f64> for #name {
            fn from_iter<T>(iter: T) -> #name
            where
                T: IntoIterator<Item = f64>,
            {
                let mut e = #name::new();
                for i in iter {
                    e.add(i);
                }
                e
            }
        }

        impl<'a> ::core::iter::FromIterator<&'a f64> for #name {
            fn from_iter<T>(iter: T) -> #name
            where
                T: IntoIterator<Item = &'a f64>,
            {
                let mut e = #name::new();
                for &i in iter {
                    e.add(i);
                }
                e
            }
        }

    })
}

/// Parse `#[statistics(method1, method2, ...)]` from a field's attributes.
fn parse_statistics_attr(field: &syn::Field) -> syn::Result<Vec<Ident>> {
    let mut methods = Vec::new();
    for attr in &field.attrs {
        if attr.path().is_ident("statistics") {
            attr.parse_nested_meta(|meta| {
                methods.push(meta.path.get_ident().ok_or_else(|| {
                    syn::Error::new_spanned(&meta.path, "expected an identifier")
                })?.clone());
                Ok(())
            })?;
        }
    }
    Ok(methods)
}
