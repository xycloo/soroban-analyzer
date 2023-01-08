use tree_sitter::Node;

use crate::types::{Block, Loop};

pub fn in_tree_match(
    code: &[u8],
    node: &Node,
    line_start: &Option<usize>,
    line_end: &Option<usize>,
    kind_match: &str,
    code_match: &str,
) -> bool {
    let node_row = node.start_position().row + 1;
    let mut in_scope = true;
    if let Some(line_start) = line_start {
        in_scope = node_row >= *line_start
    }
    if let Some(line_end) = line_end {
        in_scope = in_scope && node_row <= *line_end
    }

    if in_scope && node.start_position().row == node.end_position().row {
        let code = &code[node.start_byte()..node.end_byte()];

        if let Ok(code) = String::from_utf8(code.to_vec()) {
            if code == code_match && node.kind() == kind_match {
                return true;
            }
        } else {
        }
    }

    let count = node.child_count();
    if count != 0 {
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            if in_tree_match(
                code,
                &cursor.node(),
                line_start,
                line_end,
                kind_match,
                code_match,
            ) {
                break true;
            };
            if !cursor.goto_next_sibling() {
                break false;
            }
        }
    } else {
        false
    }
}

pub fn in_tree_matches(
    matches_count: &mut usize,
    code: &[u8],
    node: &Node,
    line_start: &Option<usize>,
    line_end: &Option<usize>,
    kind_match: &str,
    code_match: &str,
) {
    let node_row = node.start_position().row + 1;
    let mut in_scope = true;
    if let Some(line_start) = line_start {
        in_scope = node_row >= *line_start
    }
    if let Some(line_end) = line_end {
        in_scope = in_scope && node_row <= *line_end
    }

    if in_scope && node.start_position().row == node.end_position().row {
        let code = &code[node.start_byte()..node.end_byte()];

        if let Ok(code) = String::from_utf8(code.to_vec()) {
            if code == code_match && node.kind() == kind_match {
                *matches_count += 1;
            }
        } else {
        }
    }

    let count = node.child_count();
    if count != 0 {
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            in_tree_matches(
                matches_count,
                code,
                &cursor.node(),
                line_start,
                line_end,
                kind_match,
                code_match,
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    } else {
    }
}

pub fn in_tree_id_match(
    node: &Node,
    line_start: &Option<usize>,
    line_end: &Option<usize>,
    id: u16,
) -> bool {
    let node_row = node.start_position().row + 1;
    let mut in_scope = true;
    if let Some(line_start) = line_start {
        in_scope = node_row >= *line_start
    }
    if let Some(line_end) = line_end {
        in_scope = in_scope && node_row <= *line_end
    }

    if in_scope && node.start_position().row == node.end_position().row && node.kind_id() == id {
        return true;
    }

    let count = node.child_count();
    if count != 0 {
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            if in_tree_id_match(&cursor.node(), line_start, line_end, id) {
                break true;
            };
            if !cursor.goto_next_sibling() {
                break false;
            }
        }
    } else {
        false
    }
}

pub fn in_tree_loops(loops: &mut Vec<Loop>, node: &Node) {
    if node.kind_id() == 259 || node.kind_id() == 260 {
        loops.push(Loop {
            ls: node.start_position().row + 1,
            le: node.end_position().row + 1,
        });
    }

    let count = node.child_count();
    if count != 0 {
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            in_tree_loops(loops, &cursor.node());
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    } else {
    }
}

pub fn in_tree_blocks(blocks: &mut Vec<Block>, node: &Node) {
    if node.kind_id() == 272 {
        blocks.push(Block {
            ls: node.start_position().row + 1,
            le: node.end_position().row + 1,
        });
    }

    let count = node.child_count();
    if count != 0 {
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            in_tree_blocks(blocks, &cursor.node());
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    } else {
    }
}
