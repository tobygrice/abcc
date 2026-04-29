use clap::Parser;
use std::{error::Error, path::PathBuf, process::Command};

#[derive(Debug, Parser)]
#[command(author, version, about = "Another Bad C Compiler")]
struct Cli {
    /// Run the lexer. Stop before parsing
    #[arg(long, conflicts_with_all = ["parse", "codegen", "emit_assembly"])]
    lex: bool,

    /// Run the lexer and parser. Stop before assembly generation
    #[arg(long, conflicts_with_all = ["lex", "codegen", "emit_assembly"])]
    parse: bool,

    /// Perform lexing, parsing, and assembly generation. Stop before code emission
    #[arg(long, conflicts_with_all = ["lex", "parse", "emit_assembly"])]
    codegen: bool,

    /// Emit an assembly file. Stop before assembling or linking
    #[arg(short = 'S', conflicts_with_all = ["lex", "parse", "codegen"])]
    emit_assembly: bool,

    /// C source file to compile
    input_file: PathBuf,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // declare file paths using idiomatic extensions
    let src_path = &args.input_file; // with extension "c"
    let pre_path = src_path.with_extension("i");
    let asm_path = src_path.with_extension("s");
    let exe_path = src_path.with_extension("");

    /* 1. PREPROCESSING */
    // call preprocessor on src_path, produce pre_path
    let pre_res = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(src_path)
        .arg("-o")
        .arg(&pre_path)
        .status()?;

    if !pre_res.success() {
        return Err(format!("preprocessing failed with status {pre_res}").into());
    }

    /* 2. COMPILATION */
    // call compiler on pre_path, produce asm_path

    std::fs::remove_file(&pre_path)?;

    // exit early if emit_assembly flag is set
    if args.emit_assembly {
        return Ok(());
    }

    /* 3. ASSEMBLY & LINKING  */
    // call assembler and linker on asm_path, produce exe_path
    let asm_res = Command::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&exe_path)
        .status()?;

    if !asm_res.success() {
        return Err(format!("assembly failed with status {asm_res}").into());
    }

    // delete assembly file
    std::fs::remove_file(&asm_path)?;

    Ok(())
}
