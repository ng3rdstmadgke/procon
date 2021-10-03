use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use std::fmt::Write as FmtWrite;

fn main() {
    let mut r = reader();
    let n = parse::<usize>(&read_line(&mut r));
    let mut ary = split::<i64>(&read_line(&mut r));
    ary.sort();
    let min = (0..n)
        .map(|i| {(ary[i] - ary[n + i]).abs()})
        .min();
    println!("{:?}", min.unwrap());
}


#[allow(dead_code)]
fn reader() -> BufReader<Box<dyn Read>> {
    let read: Box<dyn Read> = Box::new(io::stdin());
    return BufReader::new(read);
}

#[allow(dead_code)]
fn read_line(reader: &mut BufReader<Box<dyn Read>>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).ok().unwrap();
    return line;
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
    return s.trim().parse::<T>().ok().unwrap();
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

