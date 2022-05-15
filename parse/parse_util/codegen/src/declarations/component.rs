use proc_macro::TokenStream;
use syn::{custom_keyword, parse_macro_input, Ident};

struct Component {
    emitters: Vec<Ident>,
}

pub fn parse_component(input: TokenStream) -> TokenStream {
    TokenStream::new()
}


