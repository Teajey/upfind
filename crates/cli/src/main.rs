use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    paths: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let starting_directory = std::env::current_dir().expect("cwd");

    for res in upfind::iter(&starting_directory, &args.paths) {
        let path_matches = match res {
            Ok(path_matches) => path_matches,
            Err(upfind::Error::Pattern(e)) => {
                eprintln!("encountered a pattern error: {e}");
                continue;
            }
            Err(upfind::Error::OsStrConversionFail { original_string }) => {
                eprintln!("failed to parse {} as UTF-8", original_string.display());
                continue;
            }
        };

        for p in path_matches {
            match p {
                Ok(p) => println!("{}", p.display()),
                Err(e) => eprintln!("encountered a glob error: {e}"),
            }
        }
    }
}
