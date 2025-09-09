// main.rs: The minimal kernel for the theory runtime.

mod translator;
mod file_finder;
mod codex_generator;
mod args_parser;
mod modes;
mod types;
mod runtime;

fn main() {
    let runtime = runtime::Runtime::new();
    runtime.run();
}