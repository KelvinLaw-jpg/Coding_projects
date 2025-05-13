use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};
use std::process::exit;

fn main() {
    //Make a type Vec (growerble array in rust) called args and take in the variables from user
    let args: Vec<String> = env::args().collect();

    //Check if there is an input (2 arguments in total), if not then print the usage to console and exit program
    if args.len() != 2 {
        println!("Invalid arguments!");
        println!(">> {} <sha256sum>", args[0]);
        exit(1);
    }

    //Declare all variables needed, hash, the txt file use to crack the hash and the attempts
    let wanted_hash = &args[1];
    let password_file = "src/rockyou.txt";
    let mut attempts = 1;

    println!("Attempting to crack: {}!\n", wanted_hash);

    //open the password txt file and returns a file object
    let password_list = File::open(password_file).unwrap();
    //store the password_list into a reader object using BufReader, the reason for this is to accomodate the use of lines(), so the process can be more efficient
    let reader = BufReader::new(password_list);

    //for every line in the reader, do the following as many times as there are lines in the reader object
    for line in reader.lines() {
        //make everyline a type Result<String, Error>, not just a String, for error handling purposes
        let line = line.unwrap();
        //clean up stuff like (\n, \r\n, spaces, tabs), Converts the &str back into a full String because we want an owned, independent string for further use, and convert it into byte arrays
        let password = line.trim().to_owned().into_bytes();
        //Deref the password var and get a digest using the digest() in sha256 crate, format!("{:x}",...)means: format as a lowercase hexadecimal string, so instead of [0x5e, 0x88, 0x41, ...] 
        //we get "5e884898da28047151...", will get a binary byte array instead of we dont do this
        let password_hash = format!("{:x}", Sha256::digest(&password));
        println!("[{}] {} == {}", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
        //check if password hash is equal to the input one
        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempts! {} hashes to {}!", attempts, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempts += 1;
    }

    println!("Password hash not found!");
}
