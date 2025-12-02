use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    challenge_2()
}

fn challenge_2() -> std::io::Result<()> {
    println!("Challenge 2!");
    let now: Instant = Instant::now();

    let file = File::open("input/day01_test.txt")?;
    let buf_reader: BufReader<File> = BufReader::new(file);
    
    let mut pos: i16 = 50;
    let mut zeros: i16 = 0;

    for line_result in buf_reader.lines() {
        let line = line_result?;
        let (dir, amount) = line.split_at(1);
        let amount_no: i16 = amount.parse().unwrap();
        let (new_pos, num_zeros) = determine_new_pos_and_zeros(pos, amount_no, dir);
        pos = new_pos;
        zeros += num_zeros;
    }
    let elapsed: std::time::Duration = now.elapsed();

    println!("password: {}", zeros);

    println!("elapsed: {:.2?}", elapsed);
    Ok(())
}

fn determine_new_pos_and_zeros(pos: i16, a: i16, dir: &str) -> (i16, i16) {
    let delta: i16 = if dir == "L" { -a } else { a };
    let total_pos = pos + delta;
    let crossings = if delta > 0 {
        total_pos / 100
    } else if pos == 0 {
        (100 - total_pos) / 100 - 1
    } else {
        (100 - total_pos) / 100
    };
    let new_pos = total_pos.rem_euclid(100);
    (new_pos, crossings)
}
