extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = ast.ident;
    let bname = format!("{}Builder", name);
    // pass in the span from the original name to tell the compiler where to error (if we need to).
    let bident = syn::Ident::new(&bname, name.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    let builder_fields = fields.iter().map(|f| -> proc_macro2::TokenStream {
        let name = &f.ident;
        let ty = &f.ty;
        if inner_type("Option", ty).is_some() || builder_of(f).is_some() {
            quote! { #name: #ty }
        } else {
            quote! { #name: std::option::Option<#ty> }
        }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        let (arg_type, value) =
            if let std::option::Option::Some(inner_ty) = inner_type("Option", ty) {
                // if the field is an option<T>, set an option T but store in a some.
                (inner_ty, quote! { std::option::Option::Some(#name) })
            } else if builder_of(f).is_some() {
                // if the field is a builder then type is Vec<T>, and the value in the builder is not wrapped in an option. So we shouldnt wrap the value in Some.
                (ty, quote! { #name })
            } else {
                // otherwise, we take the type used by the target, and we store in an option in the builder.
                (ty, quote! { std::option::Option::Some(#name) })
            };
        let set_method = quote! {
            pub fn #name(&mut self, #name: #arg_type) -> &mut Self {
                self.#name = #value;
                self
            }
        };

        // we need to take care not to include a builder with the same name as the set method.
        //
        // ```
        // #[derive(Builder)]
        // struct Command {
        //  #[builder](each = "env")
        //  env: Vec<String>
        //  }
        // ```
        // so here we need to check there isnt already an extend method with the same name.
        match extend_method(f) {
            std::option::Option::None => set_method,
            std::option::Option::Some((true, extend_method)) => extend_method,
            std::option::Option::Some((false, extend_method)) => {
                let expr = quote! {
                    #set_method
                    #extend_method
                };
                expr
            }
        }
    });

    // for when you call Builder::build()
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if inner_type("Option", &f.ty).is_some() || builder_of(f).is_some() {
            quote! { #name: self.#name.clone() }
        } else {
            quote! {
              #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });

    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;
        if builder_of(f).is_some() {
            quote! {
              #name: std::vec::Vec::new()
            }
        } else {
            quote! {
              #name: std::option::Option::None
            }
        }
    });

    let doc = format!(
        "implements the [builder pattern] for [`{}`]
[builder-pattern](https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder)",
        name
    );

    let expanded = quote!(
            #[doc = #doc]
            pub struct #bident {
                #(#builder_fields,)*
            }

            impl #bident {
                #(#methods)*

               pub fn build(&self) -> std::result::Result<#name, std::boxed::Box<dyn std::error::Error>> {
                   std::result::Result::Ok(#name {
                       #(#build_fields,)*
                    })
                }

            }

            impl #name {
              pub fn builder() -> #bident {
                    #bident{
                        #(#build_empty,)*
                    }
                }
            }
    );

    expanded.into()
}

fn inner_type<'a>(wrapper: &'a str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if !p.path.segments.len() == 1 || (p.path.segments[0].ident != wrapper) {
            return std::option::Option::None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_type) = p.path.segments[0].arguments {
            if inner_type.args.len() != 1 {
                return std::option::Option::None;
            }
            let inner_ty = inner_type.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return std::option::Option::Some(t);
            }
        }
    }
    std::option::Option::None
}

fn builder_of(f: &syn::Field) -> Option<&syn::Attribute> {
    for attr in &f.attrs {
        let seg = &attr.path.segments;
        if seg.len() == 1 && seg[0].ident == "builder" {
            return std::option::Option::Some(attr);
        }
    }
    std::option::Option::None
}

fn mk_err<T: quote::ToTokens>(t: T) -> Option<(bool, proc_macro2::TokenStream)> {
    std::option::Option::Some((
        false,
        syn::Error::new_spanned(t, "expected `builder(each = \"...\")`").to_compile_error(),
    ))
}

fn extend_method(f: &syn::Field) -> Option<(bool, proc_macro2::TokenStream)> {
    let name = &f.ident;
    let g = builder_of(f)?;
    let meta = match g.parse_meta() {
        std::result::Result::Ok(syn::Meta::List(mut nvs)) => {
            let meta_name = nvs.path.get_ident().unwrap();
            assert_eq!(meta_name, "builder");
            if nvs.nested.len() != 1 {
                return mk_err(nvs);
            }
            match nvs.nested.pop().unwrap().into_value() {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    if nv.path.get_ident().unwrap() != "each" {
                        return mk_err(nvs);
                    };
                    nv
                }
                meta => {
                    return mk_err(meta);
                }
            }
        }
        std::result::Result::Ok(meta) => {
            return mk_err(meta);
        }
        Err(e) => {
            return std::option::Option::Some((false, e.into_compile_error()));
        }
    };

    match &meta.lit {
        syn::Lit::Str(s) => {
            let arg = syn::Ident::new(&s.value(), s.span());
            let inner_ty = inner_type("Vec", &f.ty).unwrap();
            let method = quote! {
                    pub fn #arg(&mut self, #arg: #inner_ty) -> &mut Self {
                        self.#name.push(#arg);
                        self
                }
            };
            return std::option::Option::Some((*name.as_ref().unwrap() == arg, method));
        }
        lit => panic!("expected identifier, found {:?}", lit),
    }
}
