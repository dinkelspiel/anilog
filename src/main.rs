use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use json::JsonValue;
use json::object;
use edit;

use std::{
    io::Read,
};

enum SERIESSTATUS {
    PLANNED,
    WATCHING,
    COMPLETED,
    HOLD,
    DROPPED
}

enum MOVIESTATUS {
    PLANNED,
    COMPLETED
}

enum MANGASTATUS {
    PLANNED,
    READING,
    COMPLETED,
    HOLD,
    DROPPED
}

fn print_help() {
    println!("OPTIONS:");
    println!(" --add <CATEGORY> <NAME> <?RATING> <?STATUS>         Add anime or manga to database with optional rating and status.");
    println!(" --edit <CATEGORY> <NAME>                            Edit the notes for a specific anime or manga.");
    println!(" --editjson <CATEGORY> <NAME>                        Edit the json for a specific anime or manga.");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args.len());

    let mut data: JsonValue;

    if Path::new("./data.json").exists() {
        let mut file = File::open("data.json")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        data = json::parse(&contents).unwrap();
    } else {
        data = object! {
            series: {},
            movies: {},
            manga: {},
        };
    }

    //println!("{:?}", args);

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    if &args[1] == "--help" || &args[1] == "-h" {
        print_help();
        return Ok(());
    }

    if args[1] == "--add" {

        let category: String;

        if args[2] == "-s" || args[2] == "--series" {
            category = String::from("series");
        } else if args[2] == "-mo" || args[2] == "--movie" || args[2] == "--movies" {
            category = String::from("movies");
        } else if args[2] == "-ma" || args[2] == "--manga" {
            category = String::from("manga");
        } else {
            category = String::from("series");
        }

        if args.len() == 4 {
            data[category][&args[3]] = object!{
                name: String::from(&args[3]),
                rating: 0,
                status: 0,
                notes: ""
            };
        } else if args.len() == 5 {
            data[category][&args[3]] = object!{
                name: String::from(&args[3]),
                rating: String::from(&args[4]).parse::<i8>().unwrap(),
                status: 0,
                notes: ""
            };    
        } else if args.len() == 6 {

            let mut status: i8 = 0;
            let status_arg = &args[5].to_lowercase();
            
            if category == "series" {
                if status_arg == "planned" || status_arg == "p" {
                    status = SERIESSTATUS::PLANNED as i8;
                } else if status_arg == "watching" || status_arg == "w" {
                    status = SERIESSTATUS::WATCHING as i8;
                } else if status_arg == "completed" || status_arg == "c" {
                    status = SERIESSTATUS::COMPLETED as i8;
                } else if status_arg == "hold" || status_arg == "h" {
                    status = SERIESSTATUS::HOLD as i8;
                } else if status_arg == "dropped" || status_arg == "d" {
                    status = SERIESSTATUS::DROPPED as i8;
                }
            } else if category == "movies" {
                if status_arg == "planned" || status_arg == "p" {
                    status = MOVIESTATUS::PLANNED as i8;
                } else if status_arg == "completed" || status_arg == "c" {
                    status = MOVIESTATUS::COMPLETED as i8;
                }
            } else if category == "manga" {
                if status_arg == "planned" || status_arg == "p" {
                    status = MANGASTATUS::PLANNED as i8;
                } else if status_arg == "reading" || status_arg == "r" {
                    status = MANGASTATUS::READING as i8;
                } else if status_arg == "completed" || status_arg == "c" {
                    status = MANGASTATUS::COMPLETED as i8;
                } else if status_arg == "hold" || status_arg == "h" {
                    status = MANGASTATUS::HOLD as i8;
                } else if status_arg == "dropped" || status_arg == "d" {
                    status = MANGASTATUS::DROPPED as i8;
                }
            }

            data[category][&args[3]] = object!{
                name: String::from(&args[3]),
                rating: String::from(&args[4]).parse::<i8>().unwrap(),
                status: status,
                notes: ""
            };    
        } else {
            print_help();
            return Ok(());
        }
    } else if args[1] == "--edit" {
        if args.len() == 4 {
            let category: String;

            if args[2] == "-s" || args[2] == "--series" {
                category = String::from("series");
            } else if args[2] == "-mo" || args[2] == "--movie" || args[2] == "--movies" {
                category = String::from("movies");
            } else if args[2] == "-ma" || args[2] == "--manga" {
                category = String::from("manga");
            } else {
                category = String::from("series");
            }
            
            if data[&category].contains(args[3].clone()) {
                println!("The specified item is not present!");
                return Ok(());
            }

            let template = json::stringify(data[&category][&args[3]]["notes"].clone());
            let edited = edit::edit(template)?;
            
            data[&category][&args[3]]["notes"] = json::parse(&edited).unwrap();
        } else {
            print_help();
            return Ok(());
        }
    } else if args[1] == "--editjson" {
        if args.len() == 4 {
            let category: String;

            if args[2] == "-s" || args[2] == "--series" {
                category = String::from("series");
            } else if args[2] == "-mo" || args[2] == "--movie" || args[2] == "--movies" {
                category = String::from("movies");
            } else if args[2] == "-ma" || args[2] == "--manga" {
                category = String::from("manga");
            } else {
                category = String::from("series");
            }
            
            if data[&category].contains(args[3].clone()) {
                println!("The specified item is not present!");
                return Ok(());
            }

            let template = json::stringify(data[&category][&args[3]].clone());
            let edited = edit::edit(template)?;
            
            data[&category][&args[3]] = json::parse(&edited).unwrap();
        } else {
            print_help();
            return Ok(());
        }
    }

    // println!("{:#}", data);

    let mut file = File::create("data.json")?;
    file.write_all(json::stringify(data).as_bytes())?;
    Ok(())
}