use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &item_struct.ident;
    let generic_params = &item_struct.generics.params;

    let type_bare_fn = syn::parse_macro_input!(attr as syn::TypeBareFn);
    let params = type_bare_fn.inputs;
    let output: Box<syn::Type> = match type_bare_fn.output {
        syn::ReturnType::Default => {
            let token_stream: TokenStream = quote! {()}.into();
            Box::new(syn::parse_macro_input!(token_stream as syn::Type))
        }
        syn::ReturnType::Type(_, boxed_type) => boxed_type,
    };

    (quote! {
        #item_struct
        impl<#generic_params> Query for #struct_name<#generic_params> {
            type Param = (#params);
            type Output = #output;
            const NAME: &'static str = "snake_case(#generic_params) をしたいです。";
        }
    })
    .into()
}
