use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AlpacaMarketsMacro)]
pub fn alpaca_markets_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_alpaca_markets_macro(&ast)
}

fn impl_alpaca_markets_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let r#gen = quote! {
        impl AlpacaMarketsMAcro for #name {
            fn alpaca_markets_macro() {
                println!(
                    "Hello, Alpaca Markets! My name is {}!",
                    stringify!(#name)
                )
            }
        }
    };
    r#gen.into()
}
