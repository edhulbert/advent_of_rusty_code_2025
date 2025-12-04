use std::{fs::File, io::BufRead, io::BufReader, time::Instant};

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    println!("Part 2!");
    let now: Instant = Instant::now();
    let mut total: usize = 0;

    let reader = parse();

    for line_result in reader.lines() {
        let line: String = line_result.unwrap();
        let nums_array: Vec<usize> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(); 


        // for each potential battery slot
        // -- find the first largest in the set of the remaining batteries that allow the full complement of 12
        // e.g. find 4 nums
        // 0 1 2 3 4 5 6 7
        // 1 4 3 4 3 6 7 8
        // for 0th battery, valid set is: 
        // 0 1 2 3 4 | 5 6 7
        // 1 4 3 4 3 | 6 7 8
        // i.e. 0th index to 4th index (len- (num of batteries to switch on - current battery idx) + 1) 8 - (4 - 0)

        const NUM_OF_BATTERIES_TO_SWITCH_ON: usize = 12;
        let mut batteries_to_switch_on_idx_array: [usize; NUM_OF_BATTERIES_TO_SWITCH_ON] = [0; NUM_OF_BATTERIES_TO_SWITCH_ON];

        let num_batteries_in_bank = nums_array.len();    
        for i in 0..NUM_OF_BATTERIES_TO_SWITCH_ON {
            // find valid set of remaining batteries 
            let mut highest_idx: usize = if i != 0 { batteries_to_switch_on_idx_array[i-1] + 1 } else { 0 };
            let start_idx = highest_idx;
            let finish_idx = num_batteries_in_bank - NUM_OF_BATTERIES_TO_SWITCH_ON + i + 1;

            for j in start_idx..finish_idx {
                if nums_array[j] > nums_array[highest_idx] {
                    highest_idx = j;
                }
            }
            
            batteries_to_switch_on_idx_array[i] = highest_idx;
        }
        let mut number = 0;
        const TEN: usize = 10;
        for i in 0..batteries_to_switch_on_idx_array.len() {
            // take the len-1 - ith element, and add it x10^(i)
            let exponent: u32 = i.try_into().unwrap();
            number += nums_array[batteries_to_switch_on_idx_array[NUM_OF_BATTERIES_TO_SWITCH_ON - 1 - i]] * TEN.pow(exponent);
        }
        total += number;
    }

    println!("Took {:.2?}", now.elapsed());
    println!("Total: {}", total);


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
        total += joltage;
    }

    println!("Took {:.2?}", now.elapsed());
    println!("Total: {}", total);
}

fn parse() -> BufReader<File> {
    let file = File::open("input/day03.txt").unwrap();
    BufReader::new(file)
}
