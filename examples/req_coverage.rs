use clap::{arg, Command};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let matches = Command::new("MD Requirement Tracker")
        .version("1.0")
        .author("Ethan Lew <elew@galois.com>")
        .about("Tracks progress on MD requirements in a markdown file")
        .arg(arg!(-f --file <FILE> "Sets the input markdown file to use").required(true))
        .get_matches();

    let file_path = matches.get_one::<String>("file").unwrap();

    // Open file
    let mut file = File::open(file_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    // Regex for unchecked and checked items
    let re_checked = Regex::new(r"\[x\]").unwrap();
    let re_unchecked = Regex::new(r"\[\s\]").unwrap();

    // Find matches
    let checked_matches = re_checked.find_iter(&contents).count();
    let unchecked_matches = re_unchecked.find_iter(&contents).count();

    // Total items
    let total = checked_matches + unchecked_matches;

    // Calculate percentage
    let percentage = (checked_matches as f64 / total as f64) * 100.0;

    println!(
        "{:.2}% coverage, {}/{} requirements covered",
        percentage, checked_matches, total
    );
}
