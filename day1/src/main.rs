use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let mut list1: LinkedList<i32> = LinkedList::new();
    let mut list2: LinkedList<i32> = LinkedList::new();

    let path = Path::new("text.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines(){
        let line = line?;
        let splits: Vec<&str> = line.split_whitespace().collect();

        if let (Some(&first), Some(&second)) =  (splits.get(0), splits.get(1)) {
            list1.push_back(first.parse::<i32>().unwrap());
            list2.push_back(second.parse::<i32>().unwrap());
        }
    }
    println!("List 1: {:?}", list1);
    println!("List 2: {:?}", list2);

    Ok(())
}