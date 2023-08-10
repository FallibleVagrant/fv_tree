pub struct TreeConfig {
    pub min_trunk_height: usize,
    pub max_trunk_height: usize,
    //Trunk can branch after this height.
    pub min_height_before_trunk_can_branch: usize,
    //If the trunk hasn't branched, force it to at this height.
    pub height_when_trunk_forced_to_branch: usize,
    pub min_sticks: usize,
    pub max_sticks: usize,
}

impl TreeConfig {
    pub fn new() -> TreeConfig {
        TreeConfig {
            min_trunk_height: 3,
            max_trunk_height: 1000,
            min_height_before_trunk_can_branch: 1,
            height_when_trunk_forced_to_branch: 3,
            min_sticks: 20,
            max_sticks: 1000,
        }
    }

    //TODO: Finish panic messages.
    pub fn build(min_trunk_height: usize, max_trunk_height: usize, min_height_before_trunk_can_branch: usize, height_when_trunk_forced_to_branch: usize, min_sticks: usize, max_sticks: usize) -> TreeConfig {
        if min_trunk_height > max_trunk_height {
            panic!("");
        }

        if height_when_trunk_forced_to_branch < min_height_before_trunk_can_branch {
            panic!("");
        }

        if min_sticks > max_sticks {
            panic!("");
        }

        TreeConfig {
            min_trunk_height,
            max_trunk_height,
            min_height_before_trunk_can_branch,
            height_when_trunk_forced_to_branch,
            min_sticks,
            max_sticks,
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
