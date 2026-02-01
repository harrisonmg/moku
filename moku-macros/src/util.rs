use syn::{Attribute, Path};

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
