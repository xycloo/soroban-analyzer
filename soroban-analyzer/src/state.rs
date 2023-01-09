use crate::metric::{get_space_fns, metric_file_parse};
use crate::tree::in_tree_match;
use crate::types::{Fn, Storage};
use rust_code_analysis::node_getter::get_node;
use rust_code_analysis::{read_file_with_eol, ParserTrait};
use rust_code_analysis::{FuncSpace, RustParser};
use std::io::Write;
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

pub fn is_direct_state_access(parser: &RustParser, function: &Fn) -> bool {
    in_tree_match(
        parser.get_code(),
        &get_node(parser),
        &Some(function.ls),
        &Some(function.le),
        "field_identifier",
        "storage",
    )
}

pub fn gather_state_access(parser: &RustParser, space: FuncSpace) -> Vec<Fn> {
    let root_spaces = space.spaces;
    let mut functions = get_space_fns(&mut vec![], root_spaces);

    let mut access_state = Vec::new();
    state_access_fns(parser, functions.as_slice(), &mut access_state);

    loop {
        let mut found = false;
        for func in functions.clone() {
            for access in access_state.clone() {
                let f_match = in_tree_match(
                    parser.get_code(),
                    &get_node(parser),
                    &Some(func.ls),
                    &Some(func.le),
                    "identifier",
                    &access.name,
                );

                if f_match {
                    if !access_state.contains(&func) {
                        access_state.push(func.clone());
                    }

                    let func_idx = functions.iter().position(|x| x == &func).unwrap();
                    functions.remove(func_idx);
                    found = true;
                    break;
                }
            }
        }

        if !found {
            break;
        }
    }

    access_state
}

pub fn state_access_fns(parser: &RustParser, fns: &[Fn], found: &mut Vec<Fn>) {
    for func in fns {
        if is_direct_state_access(parser, func) {
            found.push(func.clone())
        }
    }
}

pub fn load_state_fns(
    storage: &mut Storage,
    path: &Path,
    stdout: &mut StandardStreamLock,
    write: bool,
) -> std::io::Result<()> {
    let space = metric_file_parse(path);

    let mut access_fns = gather_state_access(
        &RustParser::new(read_file_with_eol(path).unwrap().unwrap(), path, None),
        space,
    );

    if write {
        color!(stdout, White);
        writeln!(
            stdout,
            "\n\n [DEBUG] [{}] Functions found directly or indirectly accessing contract state: \n",
            path.to_str().unwrap()
        )?;

        color!(stdout, Yellow, true);

        for func in &access_fns {
            writeln!(stdout, "> {} at line {}", func.name, func.ls)?;
        }
    }

    storage.load_state_fns(&mut access_fns);

    Ok(())
}

pub fn multi_state() {}
