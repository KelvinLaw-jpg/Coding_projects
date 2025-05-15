/*
Description
----------------
In this project, you'll create a simple URL shortener that can generate and decode short URLs for long URLs. The program will take a long URL as input, generate a short URL, 
and then store the mapping between the short URL and the long URL. When a short URL is entered, the program will look up the corresponding long URL and redirect the user to 
the original website.

Pseudocode
----------------
Necessary Rust libraries and crates:
std::env for command line argument handling.
std::fs for file I/O.
rand for generating random numbers..

Parse the command line arguments to get the long URL.
Use std::env::args() to get the command line arguments.
Check that the correct number of arguments were provided.
Retrieve the long URL from the arguments.

Generate a short URL for the long URL.
Use a rand function to generate a short version of the long URL.

Store the mapping between the short URL and the long URL in a file.
Open a file for writing using std::fs::OpenOptions.
Write the mapping to the file as a comma-separated value.
Close the file.

Read the mapping file and redirect the user to the corresponding long URL.
Open the mapping file for reading using std::fs::File.
Read the file line by line using std::io::BufReader.
Parse each line to get the short URL and long URL.
If the short URL matches the user input, redirect the user to the corresponding long URL.
If no match is found, display an error message.

Handle errors gracefully and provide informative error messages to the user.
Use Result types to handle errors.
Provide informative error messages to the user.
*/
#![allow(unused)] //Tell rust compiler to not return error for any defined but unused file or crate

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use rand::{distributions::Alphanumeric, Rng};

fn main() {
    let mapping_path = "src/mapping.txt";
    let args: Vec<String> = env::args().collect(); // Make an Vector array called args and iterate through the CLI input to collect the inputs

    //Check if there is valid input count, if not, return help message
    if args.len() != 2 {
        println!("Usage: ./url_shortener <url>");
        return;
    }

    let url = &args[1]; // Make URL var and borrow the value from args 1

    //Check if it's a long URL
    if url.starts_with("http") {
        // Convert long URL to short URL.
        let mut rng = rand::thread_rng(); //Declares a mutable var rng, use thread_rng() to return a random number generator tied to the current thread
        let short_url: String = std::iter::repeat(()) //Creates an infinite iterator that repeatedly yields the unit value ()
            .map(|()| rng.sample(Alphanumeric) as char) //For each () item, Uses rng.sample(Alphanumeric) to generate a random alphanumeric character and Casts the result to a char
            .take(8) //Takes the first 8 items from the infinite iterator (i.e., generates 8 random characters)
            .collect(); //Collects those 8 characters into a String

        println!("Long URL: {}", url);
        println!("Short URL: {}", short_url);

        // Store the mapping between the short URL and the long URL in a file.
        let mut mapping_file = match OpenOptions::new().write(true).append(true).open(mapping_path) { //Creates a builder object to configure how a file is opened, use match to handle the error from the .open()
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let mapping = format!("{},{}\n", short_url, url); //formatt! is like println but to a var not to the console, also add newline after a url pair was added
        if let Err(_) = mapping_file.write_all(mapping.as_bytes()) { //convert string mapping into bytes and mapping_file.write_all() Returns a Result<(), std::io::Error>
            println!("Error writing to mapping file");
            return;
        }
    } else {
        // Read the mapping file and redirect the user to the corresponding long URL.
        let mapping_file = match File::open(mapping_path) { //check if mapping_path is a valid path
            Ok(file) => file,
            Err(_) => {
                println!("Error opening mapping file");
                return;
            }
        };
        let reader = BufReader::new(mapping_file); //load the mapping_file as a buffered reader
        for line in reader.lines() { //Do error checking on the lines
            let mapping = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading mapping file");
                    continue;
                }
            };
            let parts: Vec<&str> = mapping.split(',').collect(); //split the 2 key pair value into 2 pair with ','
            if parts.len() != 2 {
                continue;
            }
            let short = parts[0];
            let long = parts[1];
            if short == url { //match the pattern of the short url with the first part of the string in the parts array
                println!("Redirecting to {}", long);
                return;
            }
        }
        println!("Short URL not found");
    }
}
