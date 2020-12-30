use super::models::{NewPost, Post};
use super::schema::posts::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn list_posts(conn: &PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
    posts.load::<Post>(conn)
}

pub fn list_published_posts(conn: &PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
    posts.filter(published.eq(true)).limit(5).load::<Post>(conn)
}

pub fn create_post<'a>(conn: &PgConnection, other_title: &'a str, other_body: &'a str) -> Post {
    let new_post = NewPost {
        title: other_title,
        body: other_body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_post(conn: &PgConnection, update_id: i32) -> Post {
    diesel::update(posts.find(update_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {}", update_id))
}

pub fn delete_post(conn: &PgConnection, delete_id: i32) -> usize {
    diesel::delete(posts.find(delete_id))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn delete_post_by_title(conn: &PgConnection, pattern: &str) -> usize {
    diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts")
}
