use syn::{AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, Type, TypePath};

/// Check if a Path matches `name` or `moku::{name}`.
pub fn path_matches(path: &Path, name: &str) -> bool {
    let qualified_name = format!("moku::{name}");
    path.is_ident(name) || path.is_ident(&qualified_name)
}

/// Check that a Path matches `{name}<{generic}>` or `moku::{name}<{generic}>`.
///
/// If generic is None, just check that there is any single generic.
pub fn path_matches_generic(path: &Path, name: &str, generic: Option<&str>) -> bool {
    let args = match path.segments.len() {
        1 => {
            let seg = path.segments.first().unwrap();
            if seg.ident != name {
                return false;
            }

            &seg.arguments
        }
        2 => {
            let seg = path.segments.first().unwrap();
            if seg.ident != "moku" {
                return false;
            }

            let seg = path.segments.last().unwrap();
            if seg.ident != name {
                return false;
            }

            &seg.arguments
        }
        _ => return false,
    };

    match args {
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
