macro_rules! get_decloration_data_from_attrs {
    { attrs: $attrs:expr, err_span: $err_span:expr, pattern: $param:ident => $val:expr $(,)* } => {{
        use crate::declarations::{DeclInAttrKind, DeclKind};

        let mut decls = $attrs
            .into_iter()
            .filter_map(|attr| match DeclInAttrKind::from_attr(attr) {
                Some(Ok(DeclInAttrKind(DeclKind::$param))) => Some($val),
                _ => None,
            })
            .collect::<Vec<_>>();

        match decls.pop() {
            Some(_) if decls.len() != 0 => Some(Err(syn::Error::new(
                $err_span,
                format!(
                    "attribute '#[parse_util({})]' already declareted on item",
                    stringify!($param).to_lowercase(),
                ),
            ))),
            Some(v) => Some(Ok(v)),
            None => None,
        }
    }};

    { attrs: $attrs:expr, err_span: $err_span:expr, pattern: $pat:ident ($pat_param:ident) => $val:expr $(,)* } => {{
        use crate::declarations::{DeclInAttrKind, DeclKind};

        let mut decls = $attrs
            .into_iter()
            .filter_map(|attr| match DeclInAttrKind::from_attr(attr) {
                Some(Ok(DeclInAttrKind(DeclKind::$pat($pat_param)))) => Some($val),
                _ => None,
            })
            .collect::<Vec<_>>();

        match decls.pop() {
            Some(_) if decls.len() != 0 => Some(Err(syn::Error::new(
                $err_span,
                format!(
                    "attribute '#[parse_util({})]' already declareted on item",
                    stringify!($param).to_lowercase(),
                ),
            ))),
            Some(v) => Some(Ok(v)),
            None => None,
        }
    }};
}

mod component;
mod constructor;
mod event;
mod listener;

use proc_macro::TokenStream;
use syn::parse::{Lookahead1, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse_macro_input, Attribute, LitInt, Token};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(component);
    custom_keyword!(constructor);
    custom_keyword!(listener);
    custom_keyword!(event);
}

enum DeclKind {
    Component,
    Constructor,
    Listener(usize),
    Event,
}

impl ToString for DeclKind {
    fn to_string(&self) -> String {
        match self {
            Self::Component => format!("component"),
            Self::Constructor => format!("constructor"),
            Self::Listener(p) => format!("listener: {})", p),
            Self::Event => format!("event"),
        }
    }
}

impl Parse for DeclKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::component) {
            input.parse::<kw::component>()?;
            return Ok(Self::Component);
        }

        if lookahead.peek(kw::constructor) {
            input.parse::<kw::constructor>()?;
            return Ok(Self::Constructor);
        }

        if lookahead.peek(kw::listener) {
            input.parse::<kw::listener>()?;
            input.parse::<Token![:]>()?;
            let priority = input.parse::<LitInt>()?.base10_parse::<usize>()?;

            return Ok(Self::Listener(priority));
        }

        if lookahead.peek(kw::event) {
            input.parse::<kw::event>()?;
            return Ok(Self::Event);
        }

        Err(lookahead.error())
    }
}

struct DeclInAttrKind(DeclKind);

impl DeclInAttrKind {
    fn from_attr(attr: Attribute) -> Option<syn::Result<Self>> {
        match attr.path.segments.last() {
            Some(segment) if segment.ident.to_string() == "parse_util" => (),
            _ => return None,
        }

        Some(syn::parse(attr.tokens.into()))
    }
}

impl Parse for DeclInAttrKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        Ok(Self(DeclKind::parse(&content)?))
    }
}

pub fn parse_declaration(args: TokenStream, input: TokenStream) -> TokenStream {
    let decl_kind: DeclKind = parse_macro_input!(args);

    match decl_kind {
        // validation
        _ => (),
    }

    TokenStream::new()
}
