use blocks::{load_blocks, multi_state};
use clap::Parser;
use loops::{load_loops, load_state_loops};
use state::load_state_fns;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use types::{Action, Storage};
use walkdir::WalkDir;

#[macro_use]
pub mod macros;

pub mod blocks;
pub mod loops;
pub mod metric;
pub mod state;
pub mod tree;
pub mod types;

fn load_instructions() -> [Action; 2] {
    [Action::StateInLoop, Action::MultiState]
}

fn lookup_execute(
    storage: &mut Storage,
    action: &Action,
    path: &PathBuf,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    match action {
        Action::LoadStateFns => load_state_fns(storage, path, stdout, true),
        Action::LoadLoops => load_loops(storage, path, stdout, false),
        Action::LoadBlocks => load_blocks(storage, path, stdout, false),
        Action::StateInLoop => load_state_loops(storage, path, stdout),
        Action::MultiState => multi_state(storage, path, stdout, true),
        _ => Ok(()),
    }
}

fn load_storage(
    path: Option<PathBuf>,
    storage: &mut Storage,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    if let Some(path) = path {
        let instructions = [Action::LoadStateFns, Action::LoadLoops, Action::LoadBlocks];

        for action in &instructions {
            lookup_execute(storage, action, &path, stdout)?
        }
    } else {
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.ends_with(".rs") {
                let path = PathBuf::from(f_name.to_string());
                let instructions = [Action::LoadStateFns, Action::LoadLoops, Action::LoadBlocks];

                for action in &instructions {
                    lookup_execute(storage, action, &path, stdout)?
                }
            }
        }
    }

    color!(stdout, White);
    writeln!(
        stdout,
        "\n\n [DEBUG] Functions found directly or indirectly accessing contract state: \n",
    )?;

    color!(stdout, Yellow, true);

    for func in &storage.read_state_fns() {
        writeln!(stdout, "> {} at line {}", func.name, func.ls)?;
    }

    Ok(())
}

fn launch_analyzer(path: Option<PathBuf>) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    color!(stdout, White);
    writeln!(stdout, "\n\n[+] Soroban Analyzer started. Disclaimer: still under development, the tool's scope is currently very limited, expect bugs and breaking changes may occur. \nReport bugs or suggestions in the github issues page: https://github.com/xycloo/soroban-analyzer/issues.")?;

    let instructions = load_instructions();

    let mut storage = Storage::new();

    load_storage(path.clone(), &mut storage, &mut stdout)?;

    color!(stdout, Blue);
    writeln!(stdout, "\n\n\n[+] Starting checks ")?;

    if let Some(path) = path {
        for action in &instructions {
            lookup_execute(&mut storage, action, &path, &mut stdout)?
        }
    } else {
        for entry in WalkDir::new(".")
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.ends_with(".rs") {
                let path = PathBuf::from(f_name.to_string());

                for action in &instructions {
                    lookup_execute(&mut storage, action, &path, &mut stdout)?
                }
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(long = "p", conflicts_with = "all")]
    path: Option<std::path::PathBuf>,
    #[clap(long, short)]
    all: bool,
}

fn main() {
    let args = Cli::parse();

    if args.all {
        launch_analyzer(None).unwrap();
    } else {
        launch_analyzer(args.path).unwrap();
    }
}
