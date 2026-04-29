use clap::Parser;
use std::path::PathBuf;

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
}
