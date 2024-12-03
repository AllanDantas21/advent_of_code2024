use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_min_in_list(list: &Vec<i32>) -> &i32 {
    list.iter().min().unwrap()
}

fn main() -> io::Result<()> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let splits: Vec<&str> = line.split_whitespace().collect();

        if let (Some(&first), Some(&second)) = (splits.get(0), splits.get(1)) {
            list1.push(first.parse::<i32>().unwrap());
            list2.push(second.parse::<i32>().unwrap());
        }
    }

    let mut total_distance = 0;
    while !list1.is_empty() && !list2.is_empty() {
        let min1 = get_min_in_list(&list1);
        let min2 = get_min_in_list(&list2);
        let distance = (min1 - min2).abs();
        total_distance += distance;
        list1.remove(list1.iter().position(|&x| x == *min1).unwrap());
        list2.remove(list2.iter().position(|&x| x == *min2).unwrap());
    }
    println!("Total distance: {}", total_distance);
    Ok(())
}
