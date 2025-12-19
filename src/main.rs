use anyhow::Result;
use clap::Parser;
use inkwell::context::Context;
use ntu_2025fall_advanced_compiler_hw6_g36maid::transform::DSETransform;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let context = Context::create();
    let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_file(&args.input)
        .map_err(|e| anyhow::anyhow!("Failed to read input file: {}", e))?;
    let module = context
        .create_module_from_ir(memory_buffer)
        .map_err(|e| anyhow::anyhow!("Failed to parse bitcode: {}", e.to_string()))?;

    let mut _changed = false;
    for function in module.get_functions() {
        if DSETransform::run_on_function(&function) {
            _changed = true;
        }
    }

    println!("Writing output to {:?}", args.output);
    module.write_bitcode_to_path(&args.output);

    Ok(())
}
