use std::{time::Instant};
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    println!("Part 2!");
    let now: Instant = Instant::now();
    let mut total: usize = 0;

    let bytes = fs::read("input/day04.txt").unwrap();  

    let width = bytes.iter()
        .position(|&b| b == b'\n')
        .expect("No newline found");

    let mut positions: Vec<bool> = vec!();

    for &b in &bytes {
        match b {
            b'@' => positions.push(true),
            b'.' => positions.push(false),
            b'\n' => {},
            _ => panic!("unexpected byte: {}", b)
        }
    }

    let len = positions.len();
    println!("{}, {}", width, len);

    let mut any_to_remove: bool = true;

    while any_to_remove {
        let mut rolls_to_remove_indices: Vec<usize> = vec!();

        for i in 0..len {
            if is_removeable_roll(i, width, &positions) {
                rolls_to_remove_indices.push(i);
                total += 1;
            }
        }

        for roll_to_remove in 0..rolls_to_remove_indices.len() {
            positions[rolls_to_remove_indices[roll_to_remove]] = false;
        }

        if rolls_to_remove_indices.is_empty() {
            any_to_remove = false;
        }
    }

    println!("len: {}, width: {}", bytes.len(), width);
    println!("Took {:.2?}", now.elapsed());
    println!("total: {}", total);
    

}

fn part_1() {
    println!("Part 1!");
    let now: Instant = Instant::now();
    let mut total: usize = 0;

    let bytes = fs::read("input/day04.txt").unwrap();  

    let width = bytes.iter()
        .position(|&b| b == b'\n')
        .expect("No newline found");

    let mut positions: Vec<bool> = vec!();

    for &b in &bytes {
        match b {
            b'@' => positions.push(true),
            b'.' => positions.push(false),
            b'\n' => {},
            _ => panic!("unexpected byte: {}", b)
        }
    }

    let len = positions.len();
    println!("{}, {}", width, len);
    

    for i in 0..len {
        if is_removeable_roll(i, width, &positions) {
            total += 1;
        }
    }

    println!("len: {}, width: {}", bytes.len(), width);
    println!("Took {:.2?}", now.elapsed());
    println!("total: {}", total);
}

fn is_removeable_roll(i: usize, width: usize, rolls: &Vec<bool>) -> bool {
        let mut num_rolls = 0;
        let len = rolls.len();

        if !rolls[i] {
            return false;
        }

        // top left
        if i >= width && i % width != 0 && rolls[i - width - 1] {
            num_rolls += 1;
        }
        // top middle
        if i >= width && rolls[i - width] {
            num_rolls += 1;
        }
        //top right
        if i >= width && (i + 1) % width != 0 && rolls[i - width + 1] {
            num_rolls += 1;
        }
        // left middle 
        if i % width != 0 && rolls[i - 1] {
            num_rolls += 1;
        }
        // right middle 
        if (i + 1) % width != 0 && rolls[i + 1] {
            num_rolls += 1;
        }
        // bottom left
        if (i + width) < len && i % width != 0 && rolls[i + width - 1] {
            num_rolls += 1;
        }
        // bottom middle 
        if (i + width) < len && rolls[i + width] {
            num_rolls += 1;
        }
        // bottom right
        if (i + width) < len && (i + 1) % width != 0 && rolls[i + width + 1] {
            num_rolls += 1;
        }

        num_rolls < 4

}

// a b c d e
// f g h i j
// k l m n o
// p q r s t
// u v e x y

// a b c d e f g h i j k l m n o p q r s t u v e x y

// (i-width-1).   (h=7=i-5=i-width).    (i-width+1)
// (l=11=i-1).   (m=12=i).    (n=13=i+1)
// (i+width-1).   (i+width).    (i+width+1)


// a
// i = 0 
// i+1 = 1 = b 
// i-1 = -1 = n/a
// i - width - 1 = -5 = n/a
// i - width = -4 = n/a
// i - width + 1 = -3 = n/a
// i + width - 1 = 4 = e <- WRONG
// i + width = 5 = f
// i + width + 1 = 6 = g

// if (+1) {
//     // if on right edge i.e. (i+1) % width == 0 then n/a
// }

// if (-1) {
//     // if on left edge i.e. i % width == 0 then n/a
// }

// if (+ width) {
//     // if on bottom edge i.e. i + width > len then n/a
// }

// if (- width) {
//     // if on top edge i.e. i < width then n/a
// }

