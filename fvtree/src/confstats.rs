pub struct TreeConfig {
    pub min_trunk_height: usize,
    pub max_trunk_height: usize,
    pub min_height_before_trunk_branches: usize,
    pub min_sticks: usize,
    pub max_sticks: usize,
}

impl TreeConfig {
    pub fn new() -> TreeConfig {
        TreeConfig {
            min_trunk_height: 3,
            max_trunk_height: 1000,
            min_height_before_trunk_branches: 3,
            min_sticks: 20,
            max_sticks: 1000,
        }
    }
}

pub struct BranchConfig {
    pub min_sticks: usize,
    pub max_sticks: usize,
    pub min_sticks_before_branch: usize,
}

impl BranchConfig {
    pub fn new() -> BranchConfig {
        BranchConfig {
            min_sticks: 2,
            max_sticks: 4,
            min_sticks_before_branch: 2,
        }
    }
}

pub struct Config {
    pub t: TreeConfig,
    pub b: BranchConfig,
}

impl Config {
    pub fn new() -> Config {
        Config {
            t: TreeConfig::new(),
            b: BranchConfig::new(),
        }
    }
}

pub struct TreeStats {
    pub height: usize,
    pub num_sticks: usize,
    pub branch_depth: usize,
}

impl TreeStats {
    pub fn new() -> TreeStats {
        TreeStats {
            height: 0,
            num_sticks: 0,
            branch_depth: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct BranchStats {
    pub num_leaves: usize,
    pub num_sticks: usize,
    pub height: usize,
}

impl BranchStats {
    pub fn new() -> BranchStats {
        BranchStats {
            num_leaves: 0,
            num_sticks: 0,
            height: 0,
        }
    }
}

pub struct Stats {
    pub t: TreeStats,
    pub b: BranchStats,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            t: TreeStats::new(),
            b: BranchStats::new(),
        }
    }

    pub fn add_one_stick(&mut self) {
        self.t.num_sticks += 1;
        self.b.num_sticks += 1;
    }

    pub fn add_one_branch(&mut self) {
        self.t.branch_depth += 1;
    }

    pub fn sub_one_branch(&mut self) {
        self.t.branch_depth -= 1;
    }
}
