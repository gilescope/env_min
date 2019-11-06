extern crate clap;
extern crate colored;
extern crate env_min;
extern crate terminal_size;

use std::collections::HashMap;

use clap::{App, Arg};
use colored::*;
use terminal_size::{terminal_size, Width};
use env_min::*;

fn main() {
    let matches = App::new("env_min")
        .version("0.1")
        .about("Run a command multiple times, find min set of env vars that don't change the output.")
        .arg(
            Arg::with_name("command")
                .help("Command and args to call in the container")
                .required(true)
                .multiple(true),
        ).get_matches();
 
    let mut command_line = Vec::<String>::new();

    for arg in matches.values_of("command").expect("command expected") {
        command_line.push(arg.to_string());
    }

    let results: Result<HashMap<String,String>, _> = try_bisect(

        command_line,
        Options {
        },
    );

    println!();
    println!("{}", "\nResults ==>".bold());
    println!("{:#?}", results.unwrap());

//    let mut printed_height = 0;
//    match results {
//        Ok(mut transitions) => {
//            transitions.sort_by(|t1, t2| t1.after.layer.height.cmp(&t2.after.layer.height));
//
//            for transition in transitions {
//                //print previous steps...
//                if printed_height < transition.after.layer.height {
//                    for (i, layer) in histories
//                        .iter()
//                        .rev()
//                        .enumerate()
//                        .skip(printed_height + 1)
//                        .take(transition.after.layer.height - (printed_height + 1))
//                    {
//                        println!("{}: {}", i, truncate(&layer.created_by, trunc_size).bold());
//                    }
//                }
//
//                println!(
//                    "{}: {} CAUSED:\n\n {}",
//                    transition.after.layer.height,
//                    truncate(&transition.after.layer.creation_command, trunc_size).bold(),
//                    transition.after.result
//                );
//                printed_height = transition.after.layer.height;
//            }
//        }
//        Err(e) => {
//            println!("{:?}", e);
//            std::process::exit(-1);
//        }
//    }
//    //print any training steps...
//    if printed_height < histories.len() {
//        for (i, layer) in histories.iter().rev().enumerate().skip(printed_height + 1) {
//            println!("{}: {}", i, truncate(&layer.created_by, trunc_size).bold());
//        }
//    }
}
