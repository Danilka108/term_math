use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::{Attribute, Error, FnArg, ImplItemMethod, Path, Type};

struct Constructor {
    context_arg_pos: usize,
}

impl Constructor {
    fn from_method(method: ImplItemMethod) -> Option<syn::Result<Self>> {
        // Check method atributes
        let res = get_decloration_data_from_attrs! {
            attrs: method.attrs,
            err_span: method.sig.ident.span(),
            pattern: Constructor => (),
        };

        // throw error if was find same 'parse_util(constructor)' attributes
        // or skip if wasn't find 'parse_util(constructor)' attribute
        match res? {
            Ok(_) => (),
            Err(err) => return Some(Err(err)),
        }

        let args_err = syn::Result::Err(Error::new(
            method.sig.ident.span(),
            "only 'parse_util::Emitter<_>' and one 'parse_util::Context<_>' can be injectected",
        ));

        let is_context_path = |path: &Path| -> bool {
            match path.segments.first() {
                Some(segment) if segment.ident.to_string() == "parse_util" => (),
                _ => return false,
            }

            match path.segments.last() {
                Some(segment) if segment.ident.to_string() == "Context" => true,
                _ => false,
            }
        };

        let is_emitter_path = |path: &Path| -> bool {
            match path.segments.first() {
                Some(segment) if segment.ident.to_string() == "parse_util" => (),
                _ => return false,
            }

            match path.segments.last() {
                Some(segment) if segment.ident.to_string() == "Emitter" => true,
                _ => false,
            }
        };

        // Check constructor arguments.
        // Throw error if arg isn't 'parse_util::Context<_>' or 'Parse_util::Emitter<_>'
        let (mut contexts, mut errors): (Vec<_>, Vec<_>) = method
            .sig
            .inputs
            .into_iter()
            .enumerate()
            .filter_map(|(pos, arg)| match arg {
                FnArg::Typed(pat_type) => match *pat_type.ty {
                    Type::Path(type_path) if is_context_path(&type_path.path) => Some(Ok(pos)),
                    Type::Path(type_path) if is_emitter_path(&type_path.path) => None,
                    _ => Some(args_err.clone()),
                },
                _ => None,
            })
            .partition(|r| r.is_ok());

        match errors.pop() {
            Some(Err(err)) => return Some(Err(err)),
            _ => (),
        }


        None
    }
}
