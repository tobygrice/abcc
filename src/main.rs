use clap::Parser;
use std::{path::PathBuf, process::Command};

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

fn main() {
    let args = Cli::parse();

    // declare file paths using idiomatic extensions
    let src_path = &args.input_file; // with extension "c"
    let pre_path = &src_path.with_extension("i");
    let asm_path = &src_path.with_extension("s");
    let exe_path = &src_path.with_extension("");

    /* 1. PREPROCESSING */
    // call preprocessor on src_path, produce pre_path
    let preprocessing = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(&src_path)
        .arg("-o")
        .arg(&pre_path)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("preprocessing failed: {e}");
            std::process::exit(1);
        });

    if !preprocessing.success() {
        std::process::exit(1);
    }

    /* 2. COMPILATION */
    // call compiler on pre_path, produce asm_path

    std::fs::remove_file(&pre_path).unwrap_or_else(|e| {
        eprintln!("failed to delete preprocessed file: {e}");
        std::process::exit(1);
    });

    // exit early if emit_assembly flag is set
    if args.emit_assembly {
        std::process::exit(0);
    }

    /* 3. ASSEMBLY & LINKING  */
    // call assembler and linker on asm_path, produce exe_path
    let assembly = Command::new("gcc")
        .arg(asm_path)
        .arg("-o")
        .arg(exe_path)
        .status()
        .unwrap_or_else(|e| {
            eprintln!("assembly failed: {e}");
            std::process::exit(1);
        });

    if !assembly.success() {
        std::process::exit(1);
    }

    // delete assembly file
    std::fs::remove_file(&asm_path).unwrap_or_else(|e| {
        eprintln!("failed to delete assembly file: {e}");
        std::process::exit(1);
    });
}
