use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
use std::fmt;

/// Shorthand for result types returned from the Simulation simulation.
pub type Result<V, E = Error> = std::result::Result<V, E>;

#[derive(Debug)]
pub enum WhatBound {
    ExecutionEnvironment,
    ShardBlock(u32),
}

impl fmt::Display for WhatBound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhatBound::ExecutionEnvironment => write!(f, "execution environment"),
            WhatBound::ShardBlock(shard) => write!(f, "block on shard {}", shard),
        }
    }
}

/// Errors arising from the simulation.
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("no {} exists at index: {}", what, index))]
    OutOfBounds { what: WhatBound, index: usize },
    #[snafu(display("uncategorized error: {}", message))]
    Uncategorized { message: String, backtrace: Backtrace},
}