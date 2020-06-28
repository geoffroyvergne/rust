#[macro_use]
extern crate clap;
use clap::{App};

fn main() {
    println!("Hello, world!");

    /*let matches = App::new("Command line arguments application")
        .version("0.1.0")
        .author("John Doe <john.doe@email.com")
        .about("Does awesome things")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less your favorite number"))
        .get_matches();*/

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is : {}" , myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n+5),
                Err(_) =>  println!("Thats not a number! {}", s)
            }
        }
    }
}
