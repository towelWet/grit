#[path = "build/generate.rs"]
mod generate;

fn main() {
    generate::includes("src/commands/mod/Functions", "command_functions.rs", |_| {
        true
    });
    generate::includes("src/args/mod/Types", "arg_types.rs", |_| true);
    generate::methods("src/args/mod/Functions", "arg_methods.rs", |name| {
        name.starts_with("mod_GritArgs_")
    });
    generate::includes("src/args/mod/Functions", "arg_functions.rs", |name| {
        !name.starts_with("mod_GritArgs_")
    });
}
