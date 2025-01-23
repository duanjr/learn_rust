use proc_macro::TokenStream;
use syn::{self, Data};
use quote::{quote};
use syn::{DeriveInput};

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    let Data::Struct(s) = ast.data else {
        panic!("MyDefault derive macro must use in struct");
    };

    let mut field_ast = quote!();

    for (idx, f) in s.fields.iter().enumerate() {
        let (field_id, field_ty) = (&f.ident, &f.ty);

        // 逗号不要忽略了
        if field_id.is_none() {
            let field_idx = syn::Index::from(idx);
            field_ast.extend(quote! {#field_idx:<#field_ty>::default(),})
        } else {
            field_ast.extend(quote! {#field_id:<#field_ty>::default(),})
        }
    }

    quote! {
        impl Default for #id{
            fn default() -> Self{
                Self {#field_ast}
            }
        }
    }
    .into()
}