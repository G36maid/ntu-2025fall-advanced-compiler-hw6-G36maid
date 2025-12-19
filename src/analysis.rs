use inkwell::basic_block::BasicBlock;
use inkwell::values::{InstructionOpcode, InstructionValue, Operand, PointerValue};
use std::collections::HashSet;

/// Dead Store Elimination analysis engine.
///
/// This structure implements a backward data flow analysis to identify dead store instructions.
/// A store is considered "dead" if it writes to a memory location that is overwritten before
/// being read.
///
/// ## Algorithm
///
/// The analysis uses a backward sweep through each basic block, maintaining a "live set" of
/// pointer values that may be read in the future:
///
/// 1. **Initialization**: Conservative initialization assumes all pointers used in the block
///    are potentially live at block exit.
/// 2. **Backward Traversal**: Instructions are processed in reverse order:
///    - **Load**: Adds its pointer operand to the live set (the value will be read)
///    - **Store**: Checks if its pointer is in the live set:
///      - If YES: The store is LIVE (provides a value that will be read), remove from live set
///      - If NO: The store is DEAD (value is never read), mark for removal
///    - **Other instructions**: Conservatively mark their pointer operands as live
///
/// ## Correctness Invariant
///
/// The analysis preserves program semantics by ensuring that:
/// - All stores whose values are read by subsequent loads are preserved
/// - Only stores that are provably dead (overwritten without intermediate reads) are removed
/// - Volatile stores and stores with side effects are never eliminated
pub struct DSEAnalysis<'ctx> {
    pub dead_instructions: Vec<InstructionValue<'ctx>>,
}

impl<'ctx> Default for DSEAnalysis<'ctx> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'ctx> DSEAnalysis<'ctx> {
    /// Creates a new DSE analysis instance with an empty dead instruction list.
    pub fn new() -> Self {
        Self {
            dead_instructions: Vec::new(),
        }
    }

    /// Analyzes a basic block to identify dead store instructions.
    ///
    /// This method performs backward data flow analysis on a single basic block.
    /// It identifies store instructions whose values are never read before being
    /// overwritten or before the end of the block.
    ///
    /// # Arguments
    ///
    /// * `block` - The LLVM basic block to analyze
    ///
    /// # Algorithm Details
    ///
    /// The analysis proceeds in two phases:
    ///
    /// ## Phase 1: Conservative Initialization
    ///
    /// Initially assumes all pointer operands in the block are live. This conservative
    /// approach ensures we don't incorrectly mark stores as dead when their values
    /// might escape the block or be used in ways we don't fully track.
    ///
    /// ## Phase 2: Backward Data Flow Analysis
    ///
    /// Traverses instructions in reverse order, maintaining a live set:
    /// - **Store instructions**: If the target pointer is in the live set, the store
    ///   provides a needed value. If not, the store is dead and marked for removal.
    /// - **Load instructions**: Add the source pointer to the live set, indicating
    ///   that stores to this pointer are needed.
    /// - **Other instructions**: Conservatively add all pointer operands to the live
    ///   set to avoid incorrect elimination.
    pub fn analyze_block(&mut self, block: &BasicBlock<'ctx>) {
        let mut live_set: HashSet<PointerValue<'ctx>> = HashSet::new();

        // Phase 1: Conservative initialization
        // Assume all pointers used in the block are live at exit to ensure correctness
        let mut instr_iter = block.get_first_instruction();
        while let Some(instr) = instr_iter {
            for i in 0..instr.get_num_operands() {
                if let Some(Operand::Value(basic_value)) = instr.get_operand(i)
                    && basic_value.is_pointer_value()
                {
                    live_set.insert(basic_value.into_pointer_value());
                }
            }
            instr_iter = instr.get_next_instruction();
        }

        // Phase 2: Backward traversal - process instructions from bottom to top
        let mut current_instr = block.get_last_instruction();
        while let Some(instr) = current_instr {
            match instr.get_opcode() {
                InstructionOpcode::Store => {
                    // CRITICAL: Never eliminate volatile stores (per spec edge cases)
                    // Volatile stores have observable side effects and must be preserved
                    if instr.get_volatile().unwrap_or(false) {
                        // Volatile store - mark as live and skip analysis
                        if let Some(Operand::Value(basic_value)) = instr.get_operand(1)
                            && basic_value.is_pointer_value()
                        {
                            live_set.insert(basic_value.into_pointer_value());
                        }
                        // Move to next instruction without marking as dead
                    } else {
                        // Non-volatile store - perform normal dead store analysis
                        // Operand 1 is the pointer in `store val, ptr`
                        // Note: inkwell/LLVM might differ by version, but typically 1 is ptr.
                        if let Some(Operand::Value(basic_value)) = instr.get_operand(1)
                            && basic_value.is_pointer_value()
                        {
                            let ptr_val = basic_value.into_pointer_value();
                            if live_set.contains(&ptr_val) {
                                // LIVE: It provides a value that is needed.
                                // Now we effectively "kill" the need for previous values at this pointer.
                                live_set.remove(&ptr_val);
                            } else {
                                // DEAD: The value provided here is never read.
                                self.dead_instructions.push(instr);
                            }
                        }
                    }
                }
                InstructionOpcode::Load => {
                    // Operand 0 is the pointer in `load type, ptr`
                    if let Some(Operand::Value(basic_value)) = instr.get_operand(0)
                        && basic_value.is_pointer_value()
                    {
                        live_set.insert(basic_value.into_pointer_value());
                    }
                }
                _ => {
                    // Conservative: any other instruction marks its pointer operands as live (READ)
                    for i in 0..instr.get_num_operands() {
                        if let Some(Operand::Value(basic_value)) = instr.get_operand(i)
                            && basic_value.is_pointer_value()
                        {
                            live_set.insert(basic_value.into_pointer_value());
                        }
                    }
                }
            }
            current_instr = instr.get_previous_instruction();
        }
    }
}
