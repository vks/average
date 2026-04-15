use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

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
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let merge_fields = match &input.data {
        Data::Struct(s) => match &s.fields {
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
        },
        Data::Enum(_) | Data::Union(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "`#[derive(Merge)]` is only supported on structs",
            )
            .to_compile_error()
            .into();
        }
    };

    quote! {
        impl #impl_generics ::average::Merge for #name #ty_generics #where_clause {
            fn merge(&mut self, other: &Self) {
                #merge_fields
            }
        }
    }
    .into()
}
