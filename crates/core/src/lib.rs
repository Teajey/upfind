use std::{
    ffi::OsStr,
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
pub struct ReadDirErr(pub io::Error);

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
    directory: PathBuf,
    filename_to_match: &OsStr,
    prefix_match: bool,
) -> Result<impl Iterator<Item = Result<PathBuf, DirEntryErr>>, ReadDirErr> {
    // eprintln!("Reading directory: {}", directory.display());
    Ok(directory
        .read_dir()
        .map_err(ReadDirErr)?
        .filter_map(move |dir_entry| {
            dir_entry
                .map_err(DirEntryErr)
                .map(|dir_entry| {
                    let filename_to_match_bytes = filename_to_match.as_bytes();
                    let entry_filename = dir_entry.file_name();
                    // eprintln!(
                    //     "Matching {} against {}",
                    //     filename_to_match.display(),
                    //     entry_filename.display()
                    // );
                    let entry_filename_bytes = entry_filename.as_bytes();
                    let is_match = if prefix_match {
                        entry_filename_bytes.starts_with(filename_to_match_bytes)
                    } else {
                        entry_filename_bytes == filename_to_match_bytes
                    };
                    if is_match {
                        // eprintln!("Match!");
                        return Some(dir_entry.path());
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
    sub_paths_to_match.iter().flat_map(move |path_to_match| {
        starting_directory.parents().filter_map(move |directory| {
            let path = path_to_match.as_ref();
            let filename = path.file_name();
            // dbg!(&filename);
            let parent = path.parent();
            // dbg!(&parent);
            let directory = directory.join(parent?);
            Some(match_full_paths_on_directory_entries(
                directory,
                filename?,
                prefix_match,
            ))
        })
    })
}

pub use match_sub_paths_up_from_starting_directory as iter;
