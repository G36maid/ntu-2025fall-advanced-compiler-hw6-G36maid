//! # Dead Store Elimination (DSE) Pass for LLVM IR
//!
//! This library implements a Dead Store Elimination optimization pass for LLVM IR.
//! The pass uses backward data flow analysis to identify and remove redundant store
//! instructions that are overwritten before being read.
//!
//! ## Modules
//!
//! - [`analysis`]: Contains the DSE analysis engine using backward data flow
//! - [`transform`]: Contains the transformation logic for removing dead stores

pub mod analysis;
pub mod transform;
