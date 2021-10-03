use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use std::fmt::Write as FmtWrite;

fn main() {
    let mut players: Vec<[usize; 2]> = vec![];
    let mut pins: Vec<bool> = vec![];
    for (idx, line) in reader().lines().enumerate() {
        let line = line.ok().unwrap();
        if idx == 0 {
            let ary = split::<usize>(&line);
            let n = ary[0];
            let m = ary[1];
            players.resize(n, [0, 0]);
            pins.resize(m, false);
        } else {
            let s = split::<usize>(&line);
            players[idx - 1][0] = s[0];
            players[idx - 1][1] = s[1];
        }
    }

    for (idx, e) in players.iter().enumerate() {
        let a = e[0];
        let b = e[1];
        if (a + b) <= idx && (idx - a - b) < pins.len() {
            pins[idx - a - b] = true;
        }
    }

    let cnt = pins.iter().filter(|e| {**e == true}).count();
    println!("{}", cnt);
}


#[allow(dead_code)]
fn reader() -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = Box::new(io::stdin());
    return BufReader::new(read);
}

#[allow(dead_code)]
fn writer() -> BufWriter<Box<dyn Write>> {
    let write: Box<dyn Write> = Box::new(io::stdout());
    return BufWriter::new(write);
}

#[allow(dead_code)]
fn write_line(writer: &mut BufWriter<Box<dyn Write>>, line: &str) {
    writer.write(line.as_bytes()).unwrap();
    writer.write("\n".as_bytes()).unwrap();
}

#[allow(dead_code)]
fn parse<T: FromStr>(s: &str) -> T {
    return s.parse::<T>().ok().unwrap();
}

#[allow(dead_code)]
fn split<T: FromStr>(line: &str) -> Vec<T> {
    return line
        .trim()
        .split(" ")
        .map(|e| {e.parse::<T>().ok().unwrap()})
        .collect::<Vec<T>>()
}

#[allow(dead_code)]
fn split_chars(line: &str) -> Vec<char> {
    return line.trim().chars().collect::<Vec<char>>();
}

#[allow(dead_code)]
fn join<T: std::fmt::Display>(delimiter: char, arr: &[T]) -> String {
    let mut text = String::new();
    for (i, e) in arr.iter().enumerate() {
        if i > 0 {
           text.push(delimiter);
        }
        write!(text, "{}", e).unwrap();
    }
   text
}
