use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut sum: u64 = 0;

    reader.lines().for_each(|line_res| {
        let line = line_res.expect("Failed to read line");
        line.split(',').filter(|s| !s.trim().is_empty()).for_each(|range_str| {
                let range_str = range_str.trim();

                let (start_str, end_str) = range_str.split_once('-').expect("Bad range, expected start-end");
                let start: u64 = start_str.parse().expect("Bad start number");
                let end: u64 = end_str.parse().expect("Bad end number");

            sum += (start..=end).rev().filter(|&n| number_sequence_2(n)).sum::<u64>();
        });
    });
    println!("sum = {sum}"); // 1: 54641809925 2: 73694270688
    Ok(())
}

fn number_sequence_1(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 { false; }
    let (first, second) = s.split_at(s.len() / 2);
    first == second
}

fn number_sequence_2(n: u64) -> bool {
    let s = n.to_string();
    for pat_len in 1..=s.len() / 2 {
        if s.len() % pat_len != 0 { continue; }

        if s[..pat_len].repeat(s.len() / pat_len) == s { return true; }
    }
    false
}