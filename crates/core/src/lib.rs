use std::{
    ffi::OsString,
    fmt,
    path::{Path, PathBuf},
};

struct Parents<'a> {
    current: Option<&'a Path>,
}

impl<'a> Iterator for Parents<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.parent();
        Some(current)
    }
}

trait Child<'a> {
    fn parents(&'a self) -> Parents<'a>;
}

impl<'a> Child<'a> for Path {
    fn parents(&'a self) -> Parents<'a> {
        Parents {
            current: Some(self),
        }
    }
}

#[derive(Debug)]
pub enum PathError {
    Utf8ConversionFail { original_string: OsString },
    GlobPattern(glob::PatternError),
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[allow(clippy::unnecessary_debug_formatting)]
            PathError::Utf8ConversionFail { original_string } => {
                write!(f, "failed to parse {original_string:?} as UTF-8")
            }
            PathError::GlobPattern(pattern_error) => {
                write!(f, "encountered a glob pattern error: {pattern_error}")
            }
        }
    }
}

pub fn iter<P>(
    starting_directory: &Path,
    sub_paths_to_match: &[P],
) -> impl Iterator<Item = Result<impl Iterator<Item = Result<PathBuf, glob::GlobError>>, PathError>>
where
    P: AsRef<Path>,
{
    starting_directory.parents().flat_map(move |directory| {
        sub_paths_to_match.iter().map(|p| {
            let full_path = directory.join(p);
            let os_str = full_path.as_os_str();
            let string = os_str
                .to_str()
                .ok_or_else(|| PathError::Utf8ConversionFail {
                    original_string: os_str.to_os_string(),
                })?;
            glob::glob(string)
                .map_err(PathError::GlobPattern)
                .map(|paths| {
                    paths.filter(|result| {
                        result
                            .as_ref()
                            .map(|path| {
                                path.as_os_str().to_str().is_none_or(|name| {
                                    !name.ends_with("/.") && !name.ends_with("/..")
                                })
                            })
                            .unwrap_or(true)
                    })
                })
        })
    })
}
