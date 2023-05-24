use std::process;
mod db_data_modification;

fn main() {
    if let Err(err) = db_data_modification::example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
