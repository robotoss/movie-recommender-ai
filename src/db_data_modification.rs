use anyhow::Result;
use std::error::Error;

use polars::prelude::pivot::pivot;
use polars::prelude::*;

pub fn init_train_data() -> Result<(), Box<dyn Error>> {
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

    tabular_preview(&ratings, 8);

    Ok(())
}

/// We have selected just a few of the most popular movies, and users who watch the most movies, for this crosstab example.
/// The null cells in this table are the things that we would like our model to learn to fill in. Those are the places where
/// a user has not reviewed the movie yet, presumably because they have not watched it. For each user, we would like to figure
/// out which of those movies they might be most likely to enjoy.
fn tabular_preview(ratings: &DataFrame, n: usize) {
    // Group by userId and count ratings
    let user_groups = ratings
        .clone()
        .lazy()
        .groupby(["userId"])
        .agg([col("rating").count().alias("rating_count")])
        .collect()
        .expect("Can't group users");

    // Sort users by rating count and select top n
    let top_users = user_groups
        .sort(["rating_count"], true)
        .expect("Can'sort group raitong")
        .head(Some(n));

    // Group by movieId and count ratings
    let movie_groups = ratings
        .clone()
        .lazy()
        .groupby(["movieId"])
        .agg(vec![col("rating").count().alias("rating_count")])
        .collect()
        .expect("Can't group movies");

    // Sort movies by rating count and select top n
    let top_movies = movie_groups
        .sort(["rating_count"], true)
        .expect("Can't sort top movies")
        .head(Some(n));

    println!("{:?}", top_users);
    println!("{:?}", top_movies);

    let top = ratings
        .inner_join(&top_users, &["userId"], &["userId"])
        .expect("Failed oin userId")
        .inner_join(&top_movies, &["movieId"], &["movieId"])
        .expect("Failed join movieId");

    // Create a cross-tabular view of users vs movies
    let crosstab = top
        .lazy()
        .groupby(vec!["userId", "movieId"])
        .agg(vec![col("rating").sum().alias("rating_sum")])
        .collect()
        .expect("Can't set crosstab");

    println!("{:?}", crosstab);

    let out = pivot(
        &crosstab,
        ["rating_sum"],
        ["userId"],
        ["movieId"],
        false,
        None,
        None,
    )
    .expect("Can't set pivot");

    println!("{:?}", out);
}
