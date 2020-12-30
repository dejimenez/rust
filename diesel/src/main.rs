#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::io::stdin;
use std::str::FromStr;

mod db;
mod models;
mod posts_repo;
mod schema;

fn main() {
    dotenv().ok();

    let connection = db::establish_connection();

    let mut action: Action = get_action();
    loop {
        match action {
            Action::Add => {
                add(&connection);
                action = Action::Menu;
            }
            Action::List => {
                list(&connection);
                action = Action::Menu;
            }
            Action::Publish => {
                publish(&connection);
                action = Action::Menu;
            }
            Action::Delete => {
                delete(&connection);
                action = Action::Menu;
            }
            _ => {
                action = get_action();
            }
        }
    }
}

fn list(connection: &PgConnection) {
    let results = posts_repo::list_posts(&connection).unwrap();

    println!("Displaying {} posts", results.len());
    for post in results {
        println!(
            "id: {}, title: {}, body: {}, published: {}",
            post.id, post.title, post.body, post.published
        );
    }
}

fn add(connection: &PgConnection) {
    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!("\nOk! Let's write {} \n", title);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();
    let body = &body[..(body.len() - 1)];

    let post = posts_repo::create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+C";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

fn publish(connection: &PgConnection) {
    println!("What is the post id you want to publish?");
    let mut id_str = String::new();
    stdin().read_line(&mut id_str).unwrap();
    let id_str = &id_str[..(id_str.len() - 1)];
    let id: i32 = id_str.parse().unwrap();
    posts_repo::update_post(&connection, id);
}

fn delete(connection: &PgConnection) {
    println!("What is the post id you want to delete?");
    let mut id_str = String::new();
    stdin().read_line(&mut id_str).unwrap();
    let id_str = &id_str[..(id_str.len() - 1)];
    let id: i32 = id_str.parse().unwrap();
    posts_repo::delete_post(&connection, id);
}

fn delete_by_title(connection: &PgConnection) {
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    posts_repo::delete_post_by_title(&connection, title);
}

enum Action {
    Add,
    Publish,
    Delete,
    List,
    Menu,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "1" => Ok(Action::Add),
            "2" => Ok(Action::Publish),
            "3" => Ok(Action::Delete),
            "4" => Ok(Action::List),
            _ => Err(()),
        }
    }
}

fn get_action() -> Action {
    println!("What is the action you want to execute?");
    println!("1 - Add new post");
    println!("2 - Publish post");
    println!("3 - Delete post");
    println!("4 - List all");
    println!("(Press {} when finished)\n", EOF);
    let mut action_str = String::new();
    stdin().read_line(&mut action_str).unwrap();
    let action_str = &action_str[..(action_str.len() - 1)];
    match Action::from_str(&action_str.trim()) {
        Ok(action) => action,
        Err(_) => {
            println!("wrong action {}", action_str);
            get_action()
        }
    }
}
