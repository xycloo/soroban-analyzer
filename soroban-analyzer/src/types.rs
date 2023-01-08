use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Fn {
    pub name: String,
    pub ls: usize,
    pub le: usize,
}

impl Debug for Fn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("function")
            .field("name", &self.name)
            .field("lines", &(self.ls, self.le))
            .finish()
    }
}

#[derive(Clone)]
pub enum Action {
    LoadStateFns,
    LoadLoops,
    LoadBlocks,
    CheckContractFnBudget,
    StateInLoop,
    MultiState,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Loop {
    pub ls: usize,
    pub le: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub ls: usize,
    pub le: usize,
}

impl Debug for Loop {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("loop")
            .field("lines", &(self.ls, self.le))
            .finish()
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("block")
            .field("lines", &(self.ls, self.le))
            .finish()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Storage {
    state_fns: Vec<Fn>,
    loops: Vec<Loop>,
    blocks: Vec<Block>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            state_fns: vec![],
            loops: vec![],
            blocks: vec![],
        }
    }

    pub fn load_state_fns(&mut self, fns: Vec<Fn>) {
        self.state_fns = fns;
    }

    pub fn load_loops(&mut self, loops: Vec<Loop>) {
        self.loops = loops;
    }

    pub fn load_blocks(&mut self, blocks: Vec<Block>) {
        self.blocks = blocks;
    }

    pub fn read_state_fns(&self) -> &Vec<Fn> {
        &self.state_fns
    }

    pub fn read_loops(&self) -> &Vec<Loop> {
        &self.loops
    }

    pub fn read_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}
