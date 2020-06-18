extern crate rand;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use rand::Rng;

fn main() {
    let lines = lines_from_file("./wordlist-english.txt");
    let mut rng = rand::thread_rng();
    println!("Number of lines: {}", lines.len());

    for i in 0..7 {
        for j in 0..7 {
            let mut number: usize = rng.gen();
            number = number as usize % lines.len();
            print!("{} ", lines[number as usize])
        }
        println!();
    }
    pause();
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to stop. . .").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
