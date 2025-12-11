use core::{num, time};
use std::{fs::{self, File}, io::{BufRead, BufReader, Read}, time::Instant};

fn main() {
    part_1();

}

fn part_1() {
    let now = Instant::now();
    let input = fs::read_to_string("input/day06.txt").unwrap();

    let values: Vec<&str> = input
        .rsplit(" ")   
        .filter(|v| !v.is_empty())
        .collect();

    let mut operands: Vec<Operand> = vec!();
    let mut numbers: Vec<usize> = vec!();

    for value in values {
        let number = value.parse::<usize>();

        if number.is_ok() {
            numbers.push(number.unwrap());
        } else {
            let operand: Operand = match value {
                "*" => Operand::Times,
                "+" => Operand::Plus,
                _ => panic!("not a plus or times")
            };

            operands.push(operand);
        }

    }

    let number_of_calculations: usize = operands.len();

    assert!(numbers.len() % operands.len() == 0);

    let mut calcs: Vec<usize> = vec!();

    for i  in 0..numbers.len() {
        let calc_index = i % number_of_calculations;
        dbg!(calc_index);
        let potential_value = calcs.get(calc_index);
        let operand: Operand = operands[calc_index];

        let new_value = match potential_value {
            Some(&val) => calculate(val, numbers[i], operand),
            None => calculate_first(numbers[i], operand)
        };

        if calc_index >= calcs.len() {
            calcs.resize(calc_index + 1, 0);
        }

        calcs[calc_index] = new_value;

    }

    dbg!(&calcs);

    let total: usize = calcs.iter().sum();

    let time_taken = now.elapsed();
    println!("elapsed: {:?}", time_taken);
    println!("{}", total);
    

}

fn calculate(val1: usize, val2: usize, operand: Operand) -> usize {
    return match operand {
        Operand::Times => val1 * val2,
        Operand::Plus => val1 + val2
    };
}

fn calculate_first(val: usize, operand: Operand) -> usize {
    let initial_value = match operand {
        Operand::Plus => 0,
        Operand::Times => 1
    };

    calculate(val, initial_value, operand)
}

#[derive(Clone, Copy)]
enum Operand {
    Plus,
    Times
}

