use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    paths: Vec<PathBuf>,
    #[arg(short, long, action)]
    prefix_match: bool,
}

fn main() {
    let args = Args::parse();

    let starting_directory = std::env::current_dir().expect("cwd");

    for res in truff::iter(&starting_directory, &args.paths, args.prefix_match) {
        let path_matches = match res {
            Ok(path_matches) => path_matches,
            Err(e) => {
                if e.0.kind() == std::io::ErrorKind::NotFound {
                    continue;
                }

                eprintln!("failed to read a directory: {e}");
                continue;
            }
        };

        for p in path_matches {
            match p {
                Ok(p) => println!("{}", p.display()),
                Err(e) => eprintln!("failed to read an entry in a directory: {e}"),
            }
        }
    }
}
