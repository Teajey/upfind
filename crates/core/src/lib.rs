use std::{
    ffi::OsString,
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

pub enum Error {
    OsStrConversionFail { original_string: OsString },
    Pattern(glob::PatternError),
}

pub fn iter<P>(
    starting_directory: &Path,
    sub_paths_to_match: &[P],
) -> impl Iterator<Item = Result<impl Iterator<Item = Result<PathBuf, glob::GlobError>>, Error>>
where
    P: AsRef<Path>,
{
    starting_directory.parents().flat_map(move |directory| {
        sub_paths_to_match.iter().map(|p| {
            let full_path = directory.join(p);
            let os_str = full_path.as_os_str();
            let string = os_str.to_str().ok_or_else(|| Error::OsStrConversionFail {
                original_string: os_str.to_os_string(),
            })?;
            glob::glob(string).map_err(Error::Pattern).map(|paths| {
                paths.filter(|result| {
                    result
                        .as_ref()
                        .map(|path| {
                            path.as_os_str()
                                .to_str()
                                .is_none_or(|name| !name.ends_with("/.") && !name.ends_with("/.."))
                        })
                        .unwrap_or(true)
                })
            })
        })
    })
}
