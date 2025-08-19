use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// 定义属性宏 #[log]
#[proc_macro_attribute]
pub fn log(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_vis = &input_fn.vis;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            println!("--> enter {}()", stringify!(#fn_name));
            let result = (|| #fn_block)();
            println!("<-- exit {}()", stringify!(#fn_name));
            result
        }
    };

    TokenStream::from(expanded)
}
