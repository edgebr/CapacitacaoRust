extern crate proc_macro;
extern crate proc_macro_error;
#[macro_use]
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};

#[proc_macro_error]
#[proc_macro_derive(Getters)]
pub fn getters(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_str(&s).unwrap();

    // Build the impl and generate the code
    impl_getters(&ast)
}

fn impl_getters(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data: &syn::Data = &ast.data;
    let code;
    if let syn::Data::Struct(data_struct) = data {
        if let syn::Fields::Named(fields_named) = &data_struct.fields {
            let getters_code = fields_named.named.iter().map(|f| {
                let get_name = &f.ident;
                let get_type = &f.ty;
                quote! {
                    pub fn #get_name(&self) -> &#get_type {
                        &self.#get_name
                    }
                }
            });
            let setters_code = fields_named.named.iter().map(|f| {
                if let Some(ref get_name) = f.ident {
                    let get_type = &f.ty;
                    let setter_name = format_ident!("set_{}", get_name.to_string());
                    quote! {
                        pub fn #setter_name(&mut self, #get_name: #get_type) {
                            self.#get_name = #get_name;
                        }
                    }
                } else {
                    quote! {}
                }
            });
            let new_param_code = fields_named.named.iter().map(|f| {
                let get_name = &f.ident;
                let get_type = &f.ty;
                quote! {
                    #get_name: #get_type
                }
            });
            let new_attribution_code = fields_named.named.iter().map(|f| {
                let get_name = &f.ident;
                quote! {
                    #get_name
                }
            });
            code = quote! {
                impl #name {
                    pub fn new(#(#new_param_code),*) -> Self {
                        Self {
                            #(#new_attribution_code),*
                        }
                    }
                    #(#getters_code)*
                    #(#setters_code)*
                }
            };
        } else {
            abort!(
                &data_struct.fields,
                "Getters only works for structs with named fields"
            );
        }
    } else {
        abort!(&name, "Getters only works for structs");
    }
    TokenStream::from(code)
}
