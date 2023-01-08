use crate::types::Fn;
use rust_code_analysis::{get_function_spaces, read_file_with_eol};
use rust_code_analysis::{FuncSpace, SpaceKind, LANG};
use std::path::Path;

pub fn metric_file_parse(path: &Path) -> FuncSpace {
    let source = read_file_with_eol(path).unwrap().unwrap();
    let language = LANG::Rust;

    get_function_spaces(&language, source, path, None).unwrap()
}

pub fn get_space_fns(fns: &mut Vec<Fn>, parent_spaces: Vec<FuncSpace>) -> Vec<Fn> {
    for space in parent_spaces.iter() {
        if space.kind == SpaceKind::Function {
            if let Some(fname) = &space.name {
                fns.push(Fn {
                    name: fname.to_string(),
                    ls: space.start_line,
                    le: space.end_line,
                })
            }
        } else {
            get_space_fns(fns, (*space.spaces).to_vec());
        }
    }

    fns.to_vec()
}
