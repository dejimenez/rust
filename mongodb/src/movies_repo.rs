use mongodb::bson::{self, doc, Bson, Document};
use mongodb::Client;
use mongodb::Collection;

use super::models::Movie;

pub struct MoviesRepo {
    movies: Collection,
}

impl MoviesRepo {
    pub fn new(client: &Client) -> Self {
        MoviesRepo {
            movies: client.database("movies_db").collection("movies"),
        }
    }

    pub async fn insert(&self, new_movie: Movie) {
        let new_doc: Document = bson::to_document(&new_movie).unwrap();
        let insert_result = self.movies.insert_one(new_doc, None).await.unwrap();
        println!("New document ID: {:?}", insert_result.inserted_id);
        // insert_result
    }

    pub async fn insert_many(&self, new_movies: Vec<Movie>) {
        let new_docs = new_movies.iter().map(|m| bson::to_document(&m).unwrap());
        self.movies.insert_many(new_docs, None).await.unwrap();
    }

    pub async fn get_movie_by_title(&self, title: &str) -> Movie {
        let movie_bson = self
            .movies
            .find_one(
                doc! {
                    "title": title
                },
                None,
            )
            .await
            .unwrap()
            .expect("Missing 'Parasite' document.");

        bson::from_bson(Bson::Document(movie_bson)).unwrap()
    }

    pub async fn update(&self, up_movie: &Movie) {
        let new_doc: Document = bson::to_document(&up_movie).unwrap();
        let filter = doc! {
            "_id": &up_movie.id.as_ref().unwrap(),
        };
        println!("{:?}", filter);
        let update_result = self.movies.update_one(filter, new_doc, None).await.unwrap();
        println!("Updated {} document", update_result.modified_count);
    }

    pub async fn delete(&self, id: &str) {
        let delete_result = self
            .movies
            .find_one_and_delete(
                doc! {
                   "_id": bson::oid::ObjectId::with_string(id).unwrap()
                },
                None,
            )
            .await
            .unwrap();
    }
}
