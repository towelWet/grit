mod args;
mod commands;
mod exit;
mod output;
use args::GritArgs;
fn main() {
    let args = GritArgs::parse();
    match commands::dispatch(&args) {
        Ok(exit) => std::process::exit(exit.code()),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(70);
        }
    }
}
