use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use std::fmt::Write as FmtWrite;
use std::process;

fn main() {
    let mut reader: BufReader<Box<dyn Read>> = reader();
    let s1 = split::<usize>(&read_line(&mut reader));
    let n = s1[0];
    let m = s1[1];
    let mut s2 = read_line(&mut reader)
        .trim()
        .split(" ")
        .enumerate()
        .map(|(i, e)| {Animal::new(i,  e.parse::<usize>().ok().unwrap(), 0usize)})
        .collect::<Vec<Animal>>();
    s2.sort_by(|a, b| {a.legs.cmp(&b.legs)});
    // println!("{:?}", s2);

    let inc_i = s2[1].legs - s2[0].legs;
    let inc_j = s2[2].legs - s2[0].legs;
    let mut dp = vec![vec![0usize; n + 1]; n + 1];
    dp[0][0] = s2[0].legs * n;
    for i in 0..dp.len() {
        dp[i][0] = if i == 0 {dp[0][0]} else {dp[i-1][0] + inc_i};
        for j in 0..dp.len() {
            dp[i][j] = if j == 0 {dp[i][0]} else {dp[i][j - 1] + inc_j};
            if dp[i][j] > m || i + j > m as usize {
                continue;
            }
            if dp[i][j] == m {
                s2[0].num = n - i - j;
                s2[1].num = i;
                s2[2].num = j;
                s2.sort_by(|a, b| {a.index.cmp(&b.index)});
                let result = s2.iter().map(|e| e.num).collect::<Vec<usize>>();
                println!("{}", join(' ', &result));
                process::exit(0);
            }
        }
    }
    println!("-1 -1 -1");
}

struct Animal {
    index: usize,
    legs: usize,
    num: usize,
}

impl Animal {
    fn new(index: usize, legs: usize, num: usize) -> Self {
        return Animal { index, legs, num };
    }
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

