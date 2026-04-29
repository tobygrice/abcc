mod driver;

use std::process::ExitCode;

fn main() -> ExitCode {
    match driver::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
