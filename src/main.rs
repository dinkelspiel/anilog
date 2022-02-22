use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use json::JsonValue;
use json::object;
use edit;
use std::string::ToString;
use strum_macros::Display;

use std::{
    io::Read,
};

#[derive(Display, Debug)]
enum SERIESSTATUS {
    PLANNED,
    WATCHING,
    COMPLETED,
    HOLD,
    DROPPED
}

impl SERIESSTATUS {
    fn from_i8(value: i8) -> SERIESSTATUS {
        match value {
            0 => SERIESSTATUS::PLANNED,
            1 => SERIESSTATUS::WATCHING,
            2 => SERIESSTATUS::COMPLETED,
            3 => SERIESSTATUS::HOLD,
            4 => SERIESSTATUS::DROPPED,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Display, Debug)]
enum MOVIESTATUS {
    PLANNED,
    COMPLETED
}

impl MOVIESTATUS {
    fn from_i8(value: i8) -> MOVIESTATUS {
        match value {
            0 => MOVIESTATUS::PLANNED,
            1 => MOVIESTATUS::COMPLETED,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Display, Debug)]
enum MANGASTATUS {
    PLANNED,
    READING,
    COMPLETED,
    HOLD,
    DROPPED
}

impl MANGASTATUS {
    fn from_i8(value: i8) -> MANGASTATUS {
        match value {
            0 => MANGASTATUS::PLANNED,
            1 => MANGASTATUS::READING,
            2 => MANGASTATUS::COMPLETED,
            3 => MANGASTATUS::HOLD,
            4 => MANGASTATUS::DROPPED,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

fn print_help() {
    println!("SERIES STATUS:");
    println!(" --planned, -p                                       Planning to watch");
    println!(" --watching, -w                                      Currently watching");
    println!(" --completed, -c                                     Completed");
    println!(" --hold, -h                                          On Hold");
    println!(" --dropped, -d                                       Dropped");
    println!("");
    println!("MOVIE STATUS:");
    println!(" --planned, -p                                       Planning to watch");
    println!(" --completed, -c                                     Completed");
    println!("");
    println!("MANGA STATUS:");
    println!(" --planned, -p                                       Planning to watch");
    println!(" --reading, -r                                       Reading");
    println!(" --completed, -c                                     Completed");
    println!(" --hold, -h                                          On Hold");
    println!(" --dropped, -d                                       Dropped");
    println!("");
    println!("CATEGORIES:");
    println!(" --series, -s                                        Series");
    println!(" --movies, --movie, -mo                              Movies");
    println!(" --manga, -ma                                        Manga");
    println!("");
    println!("SORT:");
    println!(" --alphabetical, -a                                  Sort by alphabetical order.");
    println!(" --rating, -r                                        Sort by rating.");
    println!(" --ratingdesc, -rd                                   Sort by rating descending.");
    println!("");
    println!("STATISTIC:");
    println!(" --completed, -c                                     Number of completed items.");
    println!(" --total, -t                                         Total items in a category.");
    println!("");
    println!("OPTIONS:");
    println!(" --get <CATEGORY> <NAME>                             Get the information for a specific anime or manga in the catalogue.");
    println!(" --getstat <CATEGORY/--all> <STATISTIC>              Get a statistic under `STATISTIC` for a specific category or everything.");
    println!("x--getsort <CATEGORY/--all> <SORT> <?LENGTH/--all>   Get a specific category or everything sorted by any of the methods under `SORT`.");
    println!(" --add <CATEGORY> <NAME> <?RATING> <?STATUS>         Add anime or manga to database with optional rating and status.");
    println!(" --edit <CATEGORY> <NAME>                            Edit the notes for a specific anime or manga.");
    println!(" --editjson <CATEGORY> <NAME>                        Edit the json for a specific anime or manga.");
    println!();
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

            if String::from(&args[4]).parse::<i8>().unwrap() > 101 || String::from(&args[4]).parse::<i8>().unwrap() < 0 {
                println!("Ratings must be between 0 and 100!");
                return Ok(());
            }
        
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

            if String::from(&args[4]).parse::<i8>().unwrap() > 101 || String::from(&args[4]).parse::<i8>().unwrap() < 0 {
                println!("Ratings must be between 0 and 100!");
                return Ok(());
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
    } else if args[1] == "--get" {
        if args.len() != 4 {
            print_help();
            println!("Invalid number of arguments!");
            return Ok(());
        }

        if args[2] == "--series" || args[2] == "-s" {
            println!("Name:");
            println!(" {}", data["series"][&args[3]]["name"]);
            println!("Rating:");
            println!(" {}", data["series"][&args[3]]["rating"]);
            println!("Status:");
            let status_int = String::from(&json::stringify(data["series"][&args[3]]["status"].clone())).parse::<i8>().unwrap();
            println!(" {}", SERIESSTATUS::from_i8(status_int).to_string());
            println!("Notes:");
            println!(" {}", data["series"][&args[3]]["notes"]);
        } else if args[2] == "-mo" || args[2] == "--movie" || args[2] == "--movies" {
            println!("Name:");
            println!(" {}", data["movies"][&args[3]]["name"]);
            println!("Rating:");
            println!(" {}", data["movies"][&args[3]]["rating"]);
            println!("Status:");
            let status_int = String::from(&json::stringify(data["movies"][&args[3]]["status"].clone())).parse::<i8>().unwrap();
            println!(" {}", MOVIESTATUS::from_i8(status_int).to_string());
            println!("Notes:");
            println!(" {}", data["movies"][&args[3]]["notes"]);
        } else if args[2] == "-ma" || args[2] == "--manga" {
            println!("Name:");
            println!(" {}", data["manga"][&args[3]]["name"]);
            println!("Rating:");
            println!(" {}", data["manga"][&args[3]]["rating"]);
            println!("Status:");
            let status_int = String::from(&json::stringify(data["manga"][&args[3]]["status"].clone())).parse::<i8>().unwrap();
            println!(" {}", MANGASTATUS::from_i8(status_int).to_string());
            println!("Notes:");
            println!(" {}", data["manga"][&args[3]]["notes"]);
        }
    } else if args[1] == "--getstat" {
        if args.len() != 4 {
            print_help();
            println!("Invalid number of arguments!");
            return Ok(());
        }

        if args[2] == "--series" || args[2] == "-s" {
            if args[3] == "--completed" || args[3] == "-c" {
                println!("Indexing Series...");
                let mut completed: i16 = 0;
                for i in data["series"].entries() {
                    if i.1["status"] == SERIESSTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("You have completed {} series.", completed);
            } else if args[3] == "--total" || args[3] == "-t" {
                println!("You have {} series in the catalogue.", data["series"].entries().len());
            }
        } else if args[2] == "-mo" || args[2] == "--movie" || args[2] == "--movies" {
            if args[3] == "--completed" || args[3] == "-c" {
                println!("Indexing Movies...");
                let mut completed: i16 = 0;
                for i in data["movies"].entries() {
                    if i.1["status"] == MOVIESTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("You have completed {} movies.", completed);
            } else if args[3] == "--total" || args[3] == "-t" {
                println!("You have {} movies in the catalogue.", data["movies"].entries().len());
            }
        } else if args[2] == "-ma" || args[2] == "--manga" {
            if args[3] == "--completed" || args[3] == "-c" {
                println!("Indexing Mangas...");
                let mut completed: i16 = 0;
                for i in data["manga"].entries() {
                    if i.1["status"] == MANGASTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("You have completed {} mangas.", completed);
            } else if args[3] == "--total" || args[3] == "-t" {
                println!("You have {} mangas in the catalogue.", data["manga"].entries().len());
            }
        } else if args[2] == "--all" {
            if args[3] == "--completed" || args[3] == "-c" {
                println!("Indexing Series...");
                let mut completed: i16 = 0;
                for i in data["series"].entries() {
                    if i.1["status"] == SERIESSTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("Indexing Movies...");
                for i in data["movies"].entries() {
                    if i.1["status"] == MOVIESTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("Indexing Mangas...");
                for i in data["manga"].entries() {
                    if i.1["status"] == MANGASTATUS::COMPLETED as i8 {
                        completed += 1;
                    }
                }
                println!("You have completed {} animes and mangas.", completed);
            } else if args[3] == "--total" || args[3] == "-t" {
                println!("You have {} animes and mangas in the catalogue.", data["series"].entries().len() + data["movies"].entries().len() + data["manga"].entries().len());
            }
        }
    } else if args[1] == "--getsort" {
        println!("This is not implemented yet.");
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
            println!("Invalid number of arguments!");
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