use std::collections::HashMap;
use std::env;

use env_min::*;

fn main() {
    let mut command_line = Vec::<String>::new();

    if env::args().len() < 2 {
        eprint!("Needs a command to run");
        return;
    }

    for arg in env::args().skip(1) {
        command_line.push(arg.to_string());
    }

    let env: EnvVars = std::env::vars().collect();
    let results: Result<HashMap<String, String>, _> = minimise_env(&command_line, &env);

    print_results(results.unwrap());
}
