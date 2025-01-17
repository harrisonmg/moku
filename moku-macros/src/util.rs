use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Path, PathArguments, Type, TypePath,
};

/// Check if a Path matches `{name}` or `moku::{name}`.
pub fn path_matches(path: &Path, name: &str) -> bool {
    path.is_ident(name)
    //let qualified_name = format!("moku::{name}");
    //path.is_ident(name) || path.is_ident(&qualified_name)
}

/// Check that a Path matches `{name}<{generic}>` or `moku::{name}<{generic}>`.
///
/// If generic is None, just check that there is any single generic.
pub fn path_matches_generic(path: &Path, name: &str, generic: Option<&str>) -> bool {
    let seg = match path.segments.len() {
        1 => path.segments.first().unwrap(),
        2 => {
            let first_seg = path.segments.first().unwrap();
            if first_seg.ident != "moku" {
                return false;
            }

            path.segments.last().unwrap()
        }
        _ => return false,
    };

    if seg.ident != name {
        return false;
    }

    match &seg.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            if args.len() != 1 {
                return false;
            }

            let gen = match generic {
                None => return true,
                Some(gen) => gen,
            };

            match args.first().unwrap() {
                GenericArgument::Type(Type::Path(TypePath { path, .. })) => path.is_ident(gen),
                _ => false,
            }
        }
        _ => false,
    }
}

/// Filter a list of attributes down to those matching `{name}` or `moku::{name}`.
pub fn filter_attributes<'a>(attrs: &'a [Attribute], name: &str) -> Vec<&'a Attribute> {
    attrs
        .iter()
        .filter(move |attr| {
            let path = attr.meta.path();
            let seg = match path.segments.len() {
                1 => path.segments.first().unwrap(),
                2 => {
                    let first_seg = path.segments.first().unwrap();
                    if first_seg.ident != "moku" {
                        return false;
                    }

                    path.segments.last().unwrap()
                }
                _ => return false,
            };

            seg.ident == name
        })
        .collect()
}
