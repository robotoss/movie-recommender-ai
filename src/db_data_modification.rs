use std::error::Error;

use polars::prelude::*;

pub fn example() -> Result<(), Box<dyn Error>> {
    // Read the 'movies' DataFrame from CSV
    let movies = CsvReader::from_path("../movie-recommender-ai/movies_db/movies.csv")?
        .has_header(true)
        .infer_schema(Some(100000))
        .finish()
        .expect("Failed to open movies.csv");

    // Read the 'credits' DataFrame from CSV
    let ratings = CsvReader::from_path("../movie-recommender-ai/movies_db/ratings.csv")?
        .has_header(true)
        .finish()
        .expect("Failed to open ratings.csv");

    // Print the first row of the 'movies' DataFrame
    println!("{:?}", movies.head(Some(1)));

    // Print the first row of the 'credits' DataFrame
    println!("{:?}", ratings.head(Some(1)));

    Ok(())
}
