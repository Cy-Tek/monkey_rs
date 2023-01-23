use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(Expression)]
pub fn expression(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let mut gen = quote! {
        impl Expression for #name {}
    };

    if let syn::Data::Struct(s) = ast.data {
        let token_field = s.fields.iter().find(|field| match &field.ty {
            Type::Path(path) => path
                .path
                .segments
                .iter()
                .any(|seg| seg.ident.to_string() == "Token"),
            _ => false,
        });

        if let Some(syn::Field {
            ident: Some(ident), ..
        }) = token_field
        {
            let node_impl = quote! {
                impl Node for #name {
                    fn token_literal(&self) -> String {
                        self.#ident.literal.clone()
                    }
                }
            };

            gen.append_all(node_impl.into_iter());
        }
    }

    gen.into()
}

#[proc_macro_derive(Statement)]
pub fn statement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let mut gen = quote! {
        impl Statement for #name {}
    };

    if let syn::Data::Struct(s) = ast.data {
        let token_field = s.fields.iter().find(|field| match &field.ty {
            Type::Path(path) => path
                .path
                .segments
                .iter()
                .any(|seg| seg.ident.to_string() == "Token"),
            _ => false,
        });

        if let Some(syn::Field {
            ident: Some(ident), ..
        }) = token_field
        {
            let node_impl = quote! {
                impl Node for #name {
                    fn token_literal(&self) -> String {
                        self.#ident.literal.clone()
                    }
                }
            };

            gen.append_all(node_impl.into_iter());
        }
    }

    gen.into()
}
