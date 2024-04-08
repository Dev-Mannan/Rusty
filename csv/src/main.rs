use csv;
use std::error::Error;

fn read_fromfile(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_fromfile("./cv.csv") {
        eprintln!("{}", e);
    }
}
