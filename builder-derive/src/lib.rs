extern crate proc_macro;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;

    let bname = format!("{}Builder", name);

    // pass in the span from the original name to tell the compiler where to error (if we need to).
    let bident = syn::Ident::new(&bname, name.span());

    let expanded = quote!(
            struct #bident {
                name: Option<String>,
                shapes: Option<Vec<String>>,
                properties: Option<i64>,
            }

            impl #bident {
                fn name(&mut self, name: String) -> &mut Self {
                    self.name = Some(name);
                    self
                }

                fn shapes(&mut self, shapes: Vec<String>) -> &mut self {
                    self.shapes = Some(shapes);
                    self
                }
            }

            impl #name {
                fn builder() -> #bident {
                    #bident{
                        name: None,
                        shapes: None,
                        properties: None,
                    }
                }
            }
    );

    expanded.into()
}
