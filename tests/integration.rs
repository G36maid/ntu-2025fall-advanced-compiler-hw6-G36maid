use inkwell::context::Context;
use inkwell::values::InstructionOpcode;
use ntu_2025fall_advanced_compiler_hw6_g36maid::transform::DSETransform;
use std::path::Path;

#[test]
fn test_dse_target_optimization() {
    let context = Context::create();
    let path = Path::new("tests/fixtures/dse_target.ll");

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();

    // Verify initial count of stores
    let initial_store_count = count_stores(&module);
    assert_eq!(initial_store_count, 2, "Initial store count should be 2");

    let mut changed = false;
    for function in module.get_functions() {
        if DSETransform::run_on_function(&function) {
            changed = true;
        }
    }

    assert!(changed, "Optimization should have modified the module");

    // Verify final count
    let final_store_count = count_stores(&module);
    assert_eq!(
        final_store_count, 1,
        "Final store count should be 1 (one removed)"
    );
}

#[test]
fn test_no_op_preservation() {
    let context = Context::create();
    let path = Path::new("tests/fixtures/no_op.ll");

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();

    let initial_store_count = count_stores(&module);
    assert_eq!(initial_store_count, 2, "Initial store count should be 2");

    let mut changed = false;
    for function in module.get_functions() {
        if DSETransform::run_on_function(&function) {
            changed = true;
        }
    }

    assert!(!changed, "Optimization should NOT have modified the module");
    assert_eq!(count_stores(&module), 2, "Store count should remain 2");
}

#[test]
fn test_volatile_protection() {
    let context = Context::create();
    let path = Path::new("tests/fixtures/volatile_protect.ll");

    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_file(path).unwrap();
    let module = context.create_module_from_ir(memory_buffer).unwrap();

    // Count initial stores
    let initial_store_count = count_stores(&module);
    let initial_volatile_count = count_volatile_stores(&module);

    // test_volatile: 2 stores (1 volatile, 1 non-volatile)
    // test_multiple_volatile: 3 volatile stores
    // test_mixed_stores: 3 stores (1 volatile, 2 non-volatile)
    // Total: 8 stores, 5 volatile
    assert_eq!(
        initial_store_count, 8,
        "Initial total store count should be 8"
    );
    assert_eq!(
        initial_volatile_count, 5,
        "Initial volatile store count should be 5"
    );

    // Run DSE optimization
    let mut changed = false;
    for function in module.get_functions() {
        if DSETransform::run_on_function(&function) {
            changed = true;
        }
    }

    // Verify volatile stores are protected
    let final_volatile_count = count_volatile_stores(&module);
    assert_eq!(
        final_volatile_count, 5,
        "All 5 volatile stores MUST be preserved (never eliminated)"
    );

    // Expected eliminations:
    // test_volatile: dead non-volatile store removed? No - it's live (last store)
    // test_multiple_volatile: all volatile, all preserved
    // test_mixed_stores: first non-volatile is dead (can be removed), volatile preserved, last non-volatile live
    // So we expect 1 store to be removed (the dead non-volatile in test_mixed_stores)
    let final_store_count = count_stores(&module);
    assert!(
        final_store_count >= 5,
        "At least all 5 volatile stores must remain"
    );

    if changed {
        assert!(
            final_store_count < initial_store_count,
            "If optimization ran, some non-volatile dead stores should be removed"
        );
    }
}

fn count_stores(module: &inkwell::module::Module) -> usize {
    let mut count = 0;
    for function in module.get_functions() {
        for block in function.get_basic_blocks() {
            let mut instr_iter = block.get_first_instruction();
            while let Some(instr) = instr_iter {
                if instr.get_opcode() == InstructionOpcode::Store {
                    count += 1;
                }
                instr_iter = instr.get_next_instruction();
            }
        }
    }
    count
}

fn count_volatile_stores(module: &inkwell::module::Module) -> usize {
    let mut count = 0;
    for function in module.get_functions() {
        for block in function.get_basic_blocks() {
            let mut instr_iter = block.get_first_instruction();
            while let Some(instr) = instr_iter {
                if instr.get_opcode() == InstructionOpcode::Store
                    && instr.get_volatile().unwrap_or(false)
                {
                    count += 1;
                }
                instr_iter = instr.get_next_instruction();
            }
        }
    }
    count
}
