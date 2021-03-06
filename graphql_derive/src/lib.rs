extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate serde_derive_internals;
extern crate syn;

use proc_macro::TokenStream;
use serde_derive_internals::attr::Container;
use serde_derive_internals::Ctxt;

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
            Container::from_ast(&Ctxt::new(), ast);
            let field_name = if let Some(rename) = f.attrs.iter().find(|a| a.name() == "serde") {
                q = format!("{}--{:?}--\n", q, rename.name());
                match rename.value {
                    syn::MetaItem::NameValue(_, syn::Lit::Str(ref v, _)) => v.clone().to_string(),
                    _ => "xxx".to_string(),
                }
            } else if let Some(ref i) = f.ident {
                format!("{}", i)
            } else {
                "xxx".to_string()
            };
            q = format!("{}{}\n", q, field_name);
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
