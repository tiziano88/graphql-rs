extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

trait GraphQL {
    fn query() -> String;
}

#[proc_macro_derive(GraphQL, attributes(serde))]
pub fn graph_ql(input: TokenStream) -> TokenStream {
    let s = input.to_string();

    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_graph_ql(&ast);

    gen.parse().unwrap()
}

fn impl_graph_ql(ast: &syn::DeriveInput) -> quote::Tokens {
    let mut q = "{\n".to_string();

    let name = &ast.ident;
    if let syn::Body::Struct(ref variants) = ast.body {
        for f in variants.fields() {
            if let Some(ref i) = f.ident {
                q = format!("{}{}\n", q, i);
            }
        }
    }

    q.push_str("}\n");

    quote!{
        impl GraphQL for #name {
            fn query() -> String {
                #q.to_string()
            }
        }
    }
}
