use crate::tree::{in_tree_blocks, in_tree_matches};
use crate::types::Fn;
use crate::types::{Block, Storage};
use rust_code_analysis::read_file_with_eol;
use rust_code_analysis::{node_getter::get_node, ParserTrait, RustParser};
use std::io::Write;
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStreamLock, WriteColor};

fn get_blocks(parser: &RustParser, path: &Path) -> Vec<Block> {
    let mut blocks = Vec::new();

    in_tree_blocks(&mut blocks, &get_node(parser), &path.to_path_buf());

    blocks
}

pub fn load_blocks(
    storage: &mut Storage,
    path: &Path,
    stdout: &mut StandardStreamLock,
    write: bool,
) -> std::io::Result<()> {
    let mut blocks = get_blocks(
        &RustParser::new(read_file_with_eol(path).unwrap().unwrap(), path, None),
        path,
    );

    if write {
        color!(stdout, White);
        write!(stdout, "\n\nblocks: \n")?;

        color!(stdout, Yellow, true);
        for block in &blocks {
            writeln!(stdout, "> {:?}\n", block)?;
        }
    }

    storage.load_blocks(&mut blocks);

    Ok(())
}

fn multi_state_in_block(
    parser: &RustParser,
    state_fns: Vec<Fn>,
    blocks: Vec<Block>,
) -> Vec<(Block, Fn)> {
    let mut result = Vec::new();

    for block in blocks {
        for f in state_fns.clone() {
            let mut count = 0;
            in_tree_matches(
                &mut count,
                parser.get_code(),
                &get_node(parser),
                &Some(block.ls),
                &Some(block.le),
                "identifier",
                &f.name,
            );
            if count > 1 {
                result.push((block.clone(), f))
            }
        }
    }

    result
}

pub fn multi_state(
    storage: &mut Storage,
    path: &Path,
    stdout: &mut StandardStreamLock,
    write: bool,
) -> std::io::Result<()> {
    let multi_state_blocks = multi_state_in_block(
        &RustParser::new(read_file_with_eol(path).unwrap().unwrap(), path, None),
        storage.read_state_fns().to_vec(),
        storage.read_blocks().to_vec(),
    );

    if write {
        color!(stdout, White);

        /*        if !multi_state_blocks.is_empty() {
                    write!(
                        stdout,
                        "\n [WARNING] [{}] Blocks that use state functions multiple times: \n",
                        path.to_str().unwrap()
                    )?;
                }
        */

        for binding in multi_state_blocks {
            if binding.0.file == path {
                color!(stdout, Blue);

                write!(stdout, "\n[{}]", binding.0.file.to_str().unwrap())?;

                color!(stdout, Red, true);
                writeln!(
                    stdout,
                    " Lines {}-{}: the function `{}` defined at line {} accessed contract state and is used multiple times inside the block. It may be better to use `{}` once and save it in memory. \n",
                    binding.0.ls, binding.0.le, binding.1.name, binding.1.ls, binding.1.name
		)?;
            }
        }
    }

    Ok(())
}
