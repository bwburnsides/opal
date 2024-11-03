use crate::model::ir;
use std::collections::HashSet;

pub type BlockId = usize;

pub type Block = Vec<ir::Statement>;

// Using a Vec here instead of a HashSet for the edges because CFGs
// can be multigraphs (multiple edges to the same node).
pub type Cfg = HashSet<BlockId, Vec<BlockId>>;
