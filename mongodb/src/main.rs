use chrono::prelude::*;
use dotenv::dotenv;
use mongodb::Client;
use std::env;
use std::error::Error;

mod models;
mod movies_repo;

use models::Movie;
use movies_repo::MoviesRepo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    // A Client is needed to connect to MongoDB:
    let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;
    let movies_repo = MoviesRepo::new(&client);

    add_movie(&movies_repo).await;
    add_many_movies(&movies_repo).await;

    let mut movie = show_movie(&movies_repo, "Parasite").await;
    movie.year = 2019;
    update(&movies_repo, &movie).await;
    show_movie(&movies_repo, "Parasite").await;

    delete(&movies_repo).await;

    Ok(())
}

async fn print_databases(client: &Client) -> Result<(), Box<dyn Error>> {
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    Ok(())
}

async fn add_movie(movies_repo: &MoviesRepo) {
    let movie = Movie {
        id: None,
        title: "Parasite".to_string(),
        year: 2020,
        plot: String::from("A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure."),
        released: NaiveDate::from_ymd(2020, 2, 7).and_hms(0, 0, 0)
    };

    movies_repo.insert(movie).await;
}

async fn add_many_movies(movies_repo: &MoviesRepo) {
    let movie1 = Movie {
        id: None,
        title: "Parasite1".to_string(),
        year: 2020,
        plot: String::from("A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure."),
        released: NaiveDate::from_ymd(2020, 2, 7).and_hms(0, 0, 0)
    };
    let movie2 = Movie {
        id: None,
        title: "Parasite2".to_string(),
        year: 2020,
        plot: String::from("A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure."),
        released: NaiveDate::from_ymd(2020, 2, 7).and_hms(0, 0, 0)
    };
    movies_repo.insert_many(vec![movie1, movie2]).await;
}

async fn update(movies_repo: &MoviesRepo, movie: &Movie) {
    movies_repo.update(movie).await;
}

async fn show_movie(movies_repo: &MoviesRepo, title: &str) -> Movie {
    let movie_para = movies_repo.get_movie_by_title(title).await;
    // println!("{:?}", movie_para);
    movie_para
}

async fn delete(movies_repo: &MoviesRepo) {
    let movie_to_delete = show_movie(&movies_repo, "Parasite1").await;
    let movie_id_str = movie_to_delete.id.unwrap().to_string();
    movies_repo.delete(&movie_id_str).await;
}
