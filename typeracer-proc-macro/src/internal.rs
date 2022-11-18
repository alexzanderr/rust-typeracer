use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    Data,
    DataStruct,
    DeriveInput,
    Ident
};

// use derive_builder::Builder as DeriveBuilder;

// TODO: check if struct derives Builder, otherwise compile error
// you CANT check, ast.data doesnt contain anything related to a derive section
pub fn colorize_internal(input: TokenStream) -> TokenStream {
    // abstract syntax tree
    let ast: DeriveInput = parse_macro_input!(input);
    // dbg!(&ast);
    let struct_name = &ast.ident;
    // you cant put this string into the quote! macro, it must be Ident type
    let builder_name = format!("{}Builder", struct_name);
    // this is to be used inside quote! macro
    let builder_name_ident = Ident::new(&builder_name, struct_name.span());

    // here its rust source code
    let expanded = quote! {
        impl #struct_name {
            fn builder() -> #builder_name_ident {
                #builder_name_ident::default()
            }
        }
    };

    TokenStream::from(expanded)
}
