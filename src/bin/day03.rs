use std::{fs::File, io::BufRead, io::BufReader, time::Instant};



fn main() {
    part_1();
}

fn part_1() {
    println!("Part 1!");
    let now: Instant = Instant::now();
    let mut total: usize = 0;

    let reader = parse();

    for line_result in reader.lines() {
        let line: String = line_result.unwrap();
        let nums_array: Vec<usize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(); 

        let mut highest_first_idx = 0;

        for i in 1..nums_array.len()-1 {
            if nums_array[i] > nums_array[highest_first_idx] {
                highest_first_idx = i;
            }
        }

        let mut highest_second_idx = highest_first_idx + 1;

        for j in highest_second_idx..nums_array.len() {
            if nums_array[j] > nums_array[highest_second_idx] {
                highest_second_idx = j;
            }
        }

        let joltage = nums_array[highest_first_idx] * 10 + nums_array[highest_second_idx];
        println!("row: {}, joltage:{}", line, joltage);

        total += joltage;
    }

    println!("Took {:.2?}", now.elapsed());
    println!("Total: {}", total);
}

fn parse() -> BufReader<File> {
    let file = File::open("input/day03.txt").unwrap();
    BufReader::new(file)
}
