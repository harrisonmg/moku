use syn::{
    AngleBracketedGenericArguments, Attribute, GenericArgument, Ident, Path, PathArguments, Type,
    TypePath,
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

/// Check that the generics of a Path match the expected state enum an event types.
pub fn generics_match(path: &Path, state_enum: &Ident, event: &Option<Ident>) -> bool {
    // assume we're checking the last segment
    let seg = match path.segments.last() {
        Some(seg) => seg,
        None => return false,
    };

    match &seg.arguments {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            if args.len() == 0 {
                return false;
            }

            match &args[0] {
                GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                    if !path.is_ident(state_enum) {
                        return false;
                    }
                }
                _ => return false,
            }

            if let Some(event) = event {
                if args.len() != 2 {
                    return false;
                }

                match &args[1] {
                    GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
                        if !path.is_ident(event) {
                            return false;
                        }
                    }
                    _ => return false,
                }
            } else if args.len() != 1 {
                return false;
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
