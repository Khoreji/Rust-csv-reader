use std::error::Error;
use std::time::Instant;

extern crate csv;

fn main() {
    if let Err(e) = read_from_file("./test.csv") {
        eprintln!("{}", e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    println!("Time: {} ns", now.elapsed().as_nanos());
    Ok(())
}

