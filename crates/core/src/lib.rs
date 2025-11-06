use std::{
    fmt, io,
    os::unix::ffi::OsStrExt,
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
pub struct ReadDirErr(io::Error);

impl fmt::Display for ReadDirErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub struct DirEntryErr(io::Error);

impl fmt::Display for DirEntryErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn match_full_paths_on_directory_entries(
    directory: &Path,
    full_paths_to_match: Vec<PathBuf>,
    prefix_match: bool,
) -> Result<impl Iterator<Item = Result<PathBuf, DirEntryErr>>, ReadDirErr> {
    Ok(directory
        .read_dir()
        .map_err(ReadDirErr)?
        .filter_map(move |dir_entry| {
            dir_entry
                .map_err(DirEntryErr)
                .map(|dir_entry| {
                    for path_to_match in &full_paths_to_match {
                        let path_to_match_bytes = path_to_match.as_os_str().as_bytes();
                        let entry_path = dir_entry.path();
                        eprintln!(
                            "Matching {} against {}",
                            path_to_match.display(),
                            entry_path.display()
                        );
                        let entry_bytes = entry_path.as_os_str().as_bytes();
                        let is_match = if prefix_match {
                            entry_bytes.starts_with(path_to_match_bytes)
                        } else {
                            entry_bytes == path_to_match_bytes
                        };
                        if is_match {
                            eprintln!("Match!",);
                            return Some(entry_path);
                        }
                    }

                    None
                })
                .transpose()
        }))
}

pub fn match_sub_paths_up_from_starting_directory<P>(
    starting_directory: &Path,
    sub_paths_to_match: &[P],
    prefix_match: bool,
) -> impl Iterator<Item = Result<impl Iterator<Item = Result<PathBuf, DirEntryErr>>, ReadDirErr>>
where
    P: AsRef<Path>,
{
    starting_directory.parents().map(move |directory| {
        let full_paths_to_match = sub_paths_to_match
            .iter()
            .map(|p| directory.join(p.as_ref()))
            .collect::<Vec<_>>();
        match_full_paths_on_directory_entries(directory, full_paths_to_match, prefix_match)
    })
}

pub use match_sub_paths_up_from_starting_directory as iter;
