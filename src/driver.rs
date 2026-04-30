use abcc::{Error, Result, lex};

use clap::Parser;
use std::{fs, path::PathBuf, process::Command};

/// Command-line interface, uses clap for argument parsing.
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

/// Primary driver function. Uses GCC for preprocessing, assembly, and linking.
pub fn run() -> Result<()> {
    let args = Cli::parse();

    // declare file paths using idiomatic extensions
    let src_path = &args.input_file; // with extension "c"
    let pre_path = src_path.with_extension("i");
    let asm_path = src_path.with_extension("s");
    let exe_path = src_path.with_extension("");

    /*********************** 1. PREPROCESSING ***********************/
    // call preprocessor on src_path, produce pre_path
    let pre_res = Command::new("gcc")
        .arg("-E")
        .arg("-P")
        .arg(src_path)
        .arg("-o")
        .arg(&pre_path)
        .status()?;

    if !pre_res.success() {
        return Err(Error::GccFailure {
            status: pre_res.to_string(),
        });
    }

    /************************ 2. COMPILATION ************************/
    // call compiler on pre_path, produce asm_path
    let src = fs::read_to_string(&pre_path)?;

    let tokens = lex(&src)?;
    if args.lex {
        return Ok(());
    }

    // let ast = parse(&tokens)?;
    // if args.parse {
    //     return Ok(());
    // }

    // let asm = compile(&ast)?;
    // if args.codegen {
    //     return Ok(());
    // }

    // write assembly file
    // std::fs::write(&asm_path, asm)?;

    // exit early if emit_assembly flag is set
    if args.emit_assembly {
        return Ok(());
    }

    /******************** 3. ASSEMBLY & LINKING  ********************/
    // call assembler and linker on asm_path, produce exe_path
    let asm_res = Command::new("gcc")
        .arg(&asm_path)
        .arg("-o")
        .arg(&exe_path)
        .status()?;

    if !asm_res.success() {
        return Err(Error::GccFailure {
            status: asm_res.to_string(),
        });
    }

    // delete assembly file
    std::fs::remove_file(&asm_path)?;

    Ok(())
}
