use proc_macro::TokenStream;

#[proc_macro_derive(MyTrait)]
pub fn DeriveMyTrait(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
