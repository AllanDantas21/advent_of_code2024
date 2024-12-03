use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("text.txt")?;
    for line in content.lines() {
        println!("{}", line);
    }
    Ok(())
}