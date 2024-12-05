use io::Read;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = "input.txt";
    let mut raw: String = String::new();

    let read_result: io::Result<usize> =
        fs::File::open(input_path).and_then(|mut file| file.read_to_string(&mut raw));
    match read_result {
        Ok(sz) => println!("Successfully read all {sz} bytes."),
        Err(e) => {
            eprintln!("Failed to read file: {input_path}");
            return Err(e.into());
        }
    }

    let parse_nums: Result<Vec<i32>, _> = raw
        .trim()
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>()))
        .flatten()
        .collect();
    let Ok(nums) = parse_nums else {
        return Err(parse_nums.unwrap_err().into());
    };

    // Part One
    let mut left: Vec<i32> = nums
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (i % 2 == 0).then_some(*x))
        .collect();
    left.sort();

    let mut right: Vec<i32> = nums
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (i % 2 == 1).then_some(*x))
        .collect();
    right.sort();
    assert_eq!(left.len(), right.len());

    let mut total_dist = 0;
    let n = left.len();
    for i in 0..n {
        total_dist += left[i].abs_diff(right[i]);
    }
    println!("Total Distance: {}", total_dist);

    // Part Two
    let mut counts: HashMap<i32, usize> = HashMap::with_capacity(n);
    for x in right {
        if let Some(count) = counts.get(&x) {
            counts.insert(x, count + 1);
        } else {
            counts.insert(x, 1);
        };
    }

    let mut total_similarity: usize = 0;
    for x in left {
        if let Some(count) = counts.get(&x) {
            total_similarity += (x as usize) * count;
        } else {
            total_similarity += 0;
        };
    }

    println!("Total Similarity: {total_similarity}");

    Ok(())
}
