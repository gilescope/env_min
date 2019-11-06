//! # env_min
//!
extern crate colored;
extern crate indicatif;
extern crate rand;

use std::clone::Clone;
use std::sync::Arc;
use std::process::Command;
//use std::time::{Duration, SystemTime};


//use colored::*;
use indicatif::ProgressBar;
//use rand::Rng;
use std::collections::HashMap;


type EnvVars = HashMap<String, String>;

fn minimise_single_key<A>(key: &str, val: String, expected: &str, env: &mut EnvVars, action: &A) -> String 
where
    A: Action + 'static,
{
    let paths : Vec<_> = val.split(':').collect();
    let mut keep = vec![];

    for (i, path) in paths.iter().enumerate().rev() {
        let val: &mut String = env.entry(key.to_owned()).or_insert(val.to_string());
        val.clear();
        val.push_str(&(paths[..i].join(":") + &keep.join(":")));
        let result = action.try_container(&env);
        if let Ok(result) = result {
            if result != expected {
                keep.insert(0, path.to_string());
            }
        }
    }
    keep.join(":")
}

/// Starts the bisect operation. Calculates highest and lowest layer result and if they have
/// different outputs it starts a binary chop to figure out which layer(s) caused the change.
fn get_changes<T>(layers: &EnvVars, action: &T) -> Result<EnvVars, Box<dyn std::error::Error>>
where
    T: Action + 'static,
{
    if layers.is_empty() {
        return Ok(layers.to_owned());
    }
    let expected = action.try_container(layers)?;
    let mut best = layers.clone();
    for key in layers.keys() {
        let mut env = best.clone();
        let val = env.remove(key).unwrap();
        let result = action.try_container(&env);
        if let Ok(result) = result {
            if result == expected {
                best = env;
            }
        } else {
            // Is there some subset of this key that is ok?
            if val.contains(':') {
                let res = minimise_single_key(key, val, &expected, &mut env, action);
                let val = best.entry(key.to_owned()).or_default();
                val.replace_range(.., &res);
                // best = env;
            //    println!("X{}X", &val);
          //      println!("X{:#?}X", &env);
               // best = env;
            }
        }
    }

    return Ok(best);

//
//    let action_c = action.clone();
//    let left_handle = thread::spawn(move || action_c.try_container(&first_image_name));
//
//    let end = action.try_container(last_image_name);
//    let start = left_handle.join().expect("first layer execution error!");

//    bisect(
//        Vec::from(&layers[1..layers.len() - 1]),
//        LayerResult {
//            layer: first_layer.clone(),
//            result: start,
//        },
//        LayerResult {
//            layer: last_layer.clone(),
//            result: end,
//        },
//        action,
//    )
}
//
//fn bisect<T>(
//    history: Vec<Layer>,
//    start: LayerResult,
//    end: LayerResult,
//    action: &T,
//) -> Result<Vec<Transition>, Error>
//where
//    T: Action + 'static,
//{
//    let size = history.len();
//    if size == 0 {
//        if start.result == end.result {
//            return Err(Error::new(std::io::ErrorKind::Other, ""));
//        }
//        return Ok(vec![Transition {
//            before: Some(start.clone()),
//            after: end.clone(),
//        }]);
//    }
//
//    let half = size / 2;
//    let mid_result = LayerResult {
//        layer: history[half].clone(),
//        result: action.try_container(&history[half].image_name),
//    };
//
//    if size == 1 {
//        let mut results = Vec::<Transition>::new();
//        if *start.result != mid_result.result {
//            results.push(Transition {
//                before: Some(start.clone()),
//                after: mid_result.clone(),
//            });
//        }
//        if mid_result.result != *end.result {
//            results.push(Transition {
//                before: Some(mid_result),
//                after: end.clone(),
//            });
//        }
//        return Ok(results);
//    }
//
//    if start.result == mid_result.result {
//        action.skip((mid_result.layer.height - start.layer.height) as u64);
//        return bisect(Vec::from(&history[half + 1..]), mid_result, end, action);
//    }
//    if mid_result.result == end.result {
//        action.skip((end.layer.height - mid_result.layer.height) as u64);
//        return bisect(Vec::from(&history[..half]), start, mid_result, action);
//    }
//
//    let clone_a = action.clone();
//    let clone_b = action.clone();
//    let mid_result_c = mid_result.clone();
//
//    let hist_a = Vec::from(&history[..half]);
//
//    let left_handle = thread::spawn(move || bisect(hist_a, start, mid_result, &clone_a));
//    let right_handle =
//        thread::spawn(move || bisect(Vec::from(&history[half + 1..]), mid_result_c, end, &clone_b));
//    let mut left_results: Vec<Transition> = left_handle
//        .join()
//        .expect("left")
//        .expect("left transition err");
//
//    let right_results: Vec<Transition> = right_handle
//        .join()
//        .expect("right")
//        .expect("right transition err");
//
//    left_results.extend(right_results); // These results are sorted later...
//    Ok(left_results)
//}

trait Action: Clone + Send {
    fn try_container(&self, container_id: &EnvVars) -> Result<String, Box<dyn std::error::Error>>;
}

#[derive(Clone)]
struct DockerContainer {
    pb: Arc<ProgressBar>,
    command_line: Vec<String>,
    timeout_in_seconds: usize,
}
//
//impl DockerContainer {
//    fn new(total: u64, command_line: Vec<String>, timeout_in_seconds: usize) -> DockerContainer {
//        let pb = Arc::new(ProgressBar::new(total));
//
//        DockerContainer {
//            pb,
//            command_line,
//            timeout_in_seconds,
//        }
//    }
//}

//struct Guard<'a> { buf: &'a mut Vec<u8>, len: usize }

//impl<'a> Drop for Guard<'a> {
//    fn drop(&mut self) {
//        unsafe { self.buf.set_len(self.len); }
//    }
//}

impl Action for DockerContainer {
    fn try_container(&self, env_to_try: &EnvVars) -> Result<String, Box<dyn std::error::Error>> {
        println!("{:?}", self.command_line);

        let result = Command::new(self.command_line[0].to_owned())
            .args(self.command_line.iter().skip(1))
            .env_clear()
            .envs(env_to_try)
            .output()?;

        return Ok(String::from_utf8_lossy(&result.stdout).to_string() );
    }
}

/// Struct to hold parameters.
pub struct Options {
}


/// Create containers based on layers and run command_line against them.
/// Result is the possibly a smaller set of env vars.
pub fn try_bisect(
    command_line: Vec<String>,
    _options: Options,
) -> Result<EnvVars, Box<dyn std::error::Error>> {
    let env: EnvVars = std::env::vars().collect();

//    let mut layers = env;
//    for (key, val) in env.iter().rev().enumerate() {
//        match event.id.clone() {
//            Some(layer_name) => layers.push(Layer {
//                height: index,
//                image_name: layer_name,
//                creation_command: event.created_by.clone(),
//            }),
//            None => println!("{:<3}: {}.", index, truncate(&created, options.trunc_size)),
//        }
//    }

    let create_and_try_container =  DockerContainer{
        pb: Arc::new(ProgressBar::new(100)),
        command_line,
        timeout_in_seconds: 0
    };

    let results = get_changes(&env, &create_and_try_container);
    create_and_try_container.pb.finish_with_message("done");
    results
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::collections::HashMap;
//
//}
