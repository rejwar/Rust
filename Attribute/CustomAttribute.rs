use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn log_execution(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("Executing function...");
    item
}
