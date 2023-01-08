use blocks::{load_blocks, multi_state};
use clap::Parser;
use loops::{load_loops, load_state_loops};
use state::load_state_fns;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use types::{Action, Storage};

#[macro_use]
pub mod macros;

pub mod blocks;
pub mod loops;
pub mod metric;
pub mod state;
pub mod tree;
pub mod types;

fn load_instructions() -> [Action; 5] {
    [
        Action::LoadStateFns,
        Action::LoadLoops,
        Action::LoadBlocks,
        Action::StateInLoop,
        Action::MultiState,
    ]
}

fn lookup_execute(
    storage: &mut Storage,
    action: Action,
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

fn launch_analyzer(path: PathBuf) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    color!(stdout, White);
    writeln!(stdout, "\n\n[+] Soroban Analyzer started. Disclaimer: still under development, the tool's scope is currently very limited, expect bugs and breaking changes may occur. \nReport bugs or suggestions in the github issues page: https://github.com/xycloo/soroban-analyzer/issues.")?;

    let instructions = load_instructions();

    let mut storage = Storage::new();

    for action in instructions {
        lookup_execute(&mut storage, action, &path, &mut stdout)?
    }

    Ok(())
}

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(long = "p")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = args.path;
    launch_analyzer(path).unwrap();
}
