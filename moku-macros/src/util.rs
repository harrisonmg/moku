use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Path, PathArguments, Type, TypePath,
};

/// Check if the first segment of a Path matches `{name}` or `moku::{name}`.
pub fn path_matches(path: &Path, name: &str) -> bool {
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
}

/// Check that the generics of the last segment of a Path match a list of generics.
pub fn generics_match(path: &Path, generics: &[&str]) -> bool {
    let seg = match path.segments.last() {
        Some(seg) => seg,
        None => return false,
    };

    match &seg.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            if args.len() != generics.len() {
                return false;
            }

            for (arg, generic) in args.iter().zip(generics.iter()) {
                match arg {
                    GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                        if !path.is_ident(generic) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            true
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
