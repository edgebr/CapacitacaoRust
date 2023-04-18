extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_func(_item: TokenStream) -> TokenStream {
    "fn generated_func() -> u32 { 1 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_derive(Greetings)]
pub fn greetings(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_str(&s).unwrap();

    // Build the impl and generate the code
    impl_hello(&ast)
}

fn impl_hello(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let code = quote! {
        impl Greetings for #name {
            fn hello() {
                println!("Hello, I am a {}", stringify!(#name));
            }
        }
    };
    TokenStream::from(code)
}
