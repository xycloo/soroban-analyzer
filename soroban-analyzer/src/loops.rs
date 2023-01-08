use rust_code_analysis::read_file_with_eol;
use rust_code_analysis::{node_getter::get_node, ParserTrait, RustParser};
use std::io::Write;
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

use crate::tree::in_tree_match;
use crate::types::Storage;
use crate::{
    tree::in_tree_loops,
    types::{Fn, Loop},
};

fn get_loops(parser: &RustParser) -> Vec<Loop> {
    let mut loops = Vec::new();

    in_tree_loops(&mut loops, &get_node(parser));

    loops
}

pub fn load_loops(
    storage: &mut Storage,
    path: &Path,
    stdout: &mut StandardStreamLock,
    write: bool,
) -> std::io::Result<()> {
    let loops = get_loops(&RustParser::new(
        read_file_with_eol(path).unwrap().unwrap(),
        path,
        None,
    ));

    if write {
        color!(stdout, White);
        write!(stdout, "\n\nloops: \n")?;

        color!(stdout, Yellow);
        for l in &loops {
            writeln!(stdout, "|- {:?}", l)?;
        }
    }

    storage.load_loops(loops);

    Ok(())
}

fn state_in_loops(parser: &RustParser, state_fns: Vec<Fn>, loops: Vec<Loop>) -> Vec<Loop> {
    let mut state_loops = Vec::new();
    for l in loops {
        for f in state_fns.clone() {
            if in_tree_match(
                parser.get_code(),
                &get_node(parser),
                &Some(l.ls),
                &Some(l.le),
                "identifier",
                &f.name,
            ) {
                state_loops.push(l.clone());
            }
        }
    }

    state_loops
}

pub fn load_state_loops(
    storage: &mut Storage,
    path: &Path,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let loops = state_in_loops(
        &RustParser::new(read_file_with_eol(path).unwrap().unwrap(), path, None),
        storage.read_state_fns().to_vec(),
        storage.read_loops().to_vec(),
    );

    color!(stdout, White);
    write!(stdout, "\n\n[+] Loops that access state: \n")?;

    color!(stdout, Red, true);

    for l in loops {
        writeln!(stdout, "\n[-] Line {}: loop accesses contract state, it could lead to breaking the budget as state functions are more expensive. Make sure you trust the range and that accessing or modifying the state within the loop is necessary. \n", l.ls)?;
    }

    Ok(())
}
