//! # env_min
//!
//! Compute min env vars required to create same output.

use std::clone::Clone;
use std::collections::HashMap;
use std::process::Command;

pub type EnvVars = HashMap<String, String>;

fn minimise_single_key(
    key: &str,
    val: &str,
    expected: &str,
    env: &mut EnvVars,
    action: &CommandRunner,
    separator: char,
) -> String {
    let paths: Vec<_> = val.split(separator).collect();
    let mut keep: Vec<String> = vec![];

    for (i, path) in paths.iter().enumerate().rev() {
        let val: &mut String = env.entry(key.to_owned()).or_insert(val.to_string());
        val.clear();
        val.push_str(
            &(paths[..i].join(&separator.to_string())
                + &keep.join(&separator.to_string())),
        );
        //println!("trial: {}", &val);
        let result = action.run(&env);
        //println!("trial result: {:?}", &result);
        if let Ok(result) = result {
            if &result != expected {
                keep.insert(0, path.to_string());
            }
        } else {
            keep.insert(0, path.to_string());
        }
    }
    keep.join(&separator.to_string())
}

/// Result is the possibly a smaller set of env vars.
pub fn minimise_env(command_line: &Vec<String>, layers: &EnvVars) -> Result<EnvVars, Box<dyn std::error::Error>> {
    let action = CommandRunner::new((*command_line).clone());
    if layers.is_empty() {
        return Ok(layers.to_owned());
    }
    let expected = action.run(layers)?;
    let mut best = layers.clone();
    for key in layers.keys() {
        let mut trial = best.clone();
        let val = trial.remove(key).unwrap();
        let result = action.run(&trial);
        if let Ok(result) = result {
            if result == expected {
                best = trial;
            }
        } else {
            // Is there some subset of this key that is ok?
            for sep_char in &[':', ';'] {
                let sep_count = val.chars().filter(|ch| ch == sep_char).count();
                if sep_count > 1 {
                    //println!("going to min {} key = {}<END> ", &key, &val);
                    let res = minimise_single_key(
                        key, &val, &expected, &mut trial, &action, *sep_char);
                    //print!("{}", &res);
                    let val = best.entry(key.to_owned()).or_default();
                    val.replace_range(.., &res);
                }
            }
        }
    }

    return Ok(best);
}

#[derive(Clone)]
struct CommandRunner {
    command_line: Vec<String>,
    timeout_in_seconds: usize,
}

impl CommandRunner {
    fn new(command_line: Vec<String>) -> CommandRunner {
        CommandRunner {
            command_line,
            timeout_in_seconds: 0,
        }
    }

    fn run(&self, env_to_try: &EnvVars) -> Result<String, Box<dyn std::error::Error>> {
        //println!("{:?}", self.command_line);

        let result = Command::new(self.command_line[0].to_owned())
            .args(self.command_line.iter().skip(1))
            .env_clear()
            .envs(env_to_try)
            .output()?;

        let mut output = String::from_utf8_lossy(&result.stdout).to_string();
        output.push_str(&String::from_utf8_lossy(&result.stderr));
        return Ok(output);
    }
}

pub fn print_results(results: EnvVars) {
    println!();
    for (k, v) in results {
        println!("{}={}", k, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_no_env_vars_should_return_nothing() {
        let cmd = CommandRunner::new(vec!["env".to_owned()]);
        let envvars = HashMap::<String, String>::new();
        let result = cmd.run(&envvars).unwrap();

        // With path gone it should not be able to find any env vars...
        assert_eq!(result, "");
    }

    #[test]
    fn smaller() {
        let cmd = vec!["rustc".to_owned()];
        let prev: EnvVars = std::env::vars().collect();
        assert!(prev.len() > 0);
        let min = minimise_env(&cmd, &prev).expect("min env");
        assert!(min.len() > 0);

        assert_ne!(prev, min);

        println!("fin: {:#?}", min);

        // Running again we should not be able to improve things.
        let min_min = minimise_env(&cmd, &min).expect("min env");
        assert_eq!(min, min_min);
    }

    /// The osx/unix `env` is an Identity command.
    #[test]
    fn set_should_never_be_minimisable() {
        let cmd = vec!["env".to_owned()];
        let prev: EnvVars = std::env::vars().collect();
        assert!(prev.len() > 0);
        let min = minimise_env(&cmd, &prev).expect("min env");

        assert_eq!(format!("{:#?}", prev), format!("{:#?}", min));
    }
}