use crate::analysis::DSEAnalysis;
use inkwell::values::FunctionValue;

/// Dead Store Elimination transformation pass.
///
/// This structure applies the DSE optimization by removing instructions identified
/// as dead by the analysis phase. The transformation modifies the LLVM IR in-place.
pub struct DSETransform;

impl DSETransform {
    /// Runs the DSE pass on a single function.
    ///
    /// This method orchestrates the complete DSE optimization for one function:
    /// 1. Creates a DSE analysis instance
    /// 2. Analyzes all basic blocks in the function
    /// 3. Removes all identified dead store instructions
    ///
    /// # Arguments
    ///
    /// * `function` - The LLVM function to optimize
    ///
    /// # Returns
    ///
    /// Returns `true` if the function was modified (dead stores were removed),
    /// `false` if no changes were made.
    ///
    /// # Implementation Notes
    ///
    /// This function safely removes instructions from the LLVM IR by:
    /// - Collecting all dead instructions during analysis before any modifications
    /// - Removing each instruction exactly once using `remove_from_basic_block()`
    /// - Only removing instructions verified to be dead (preserving program semantics)
    pub fn run_on_function<'ctx>(function: &FunctionValue<'ctx>) -> bool {
        let mut analysis = DSEAnalysis::new();

        // Analyze all blocks
        for block in function.get_basic_blocks() {
            analysis.analyze_block(&block);
        }

        if analysis.dead_instructions.is_empty() {
            return false;
        }

        // Remove dead instructions
        for instr in analysis.dead_instructions {
            instr.remove_from_basic_block();
        }

        true
    }
}
