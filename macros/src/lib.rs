#[allow(unused_extern_crates)]
extern crate proc_macro;
extern crate serde_json;
extern crate tokio;

use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn queue_job(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemFn);
    let name = input.sig.ident;
    let values = input.sig.inputs;
    let outputs = input.sig.output;
    let intern = input.block;
    let extended = quote! {
        fn #name(value:String) -> Result<tokio::task::JoinHandle<Result<(),String>>,serde_json::Error> {
            async fn _intern(#values) #outputs {
                #intern
            }
            let mut val = serde_json::from_str(&value)?;
            return Ok(tokio::spawn(_intern(val)));
        }
    };

    TokenStream::from(extended)
}
