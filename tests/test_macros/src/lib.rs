use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, ItemFn},
};

#[proc_macro_attribute]
pub fn instrumented_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let block = &input.block;
    let sig = &input.sig;
    let attrs = &input.attrs;
    let vis = &input.vis;

    let output = quote! {
        #(#attrs)*
        #vis #sig {
            utils::setup();
            let span = tracing::info_span!(#fn_name_str);
            let _g = span.enter();
            #block
        }
    };

    output.into()
}
