use std::path::Path;
use openapi::{apis::{self, configuration::ApiKey},models};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserErrors {
    #[error("No Users found")]
    NoUsers,
    #[error("Too many users found")]
    TooManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfgile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppsercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User { user: String }

fn load_users() -> anyhow::Result<Vec<User>> {
    let mypath = Path::new("users.json");
    let raw_text = std::fs::read_to_string(mypath)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    anyhow::bail!("Erorororororororor");
    Ok(users)
}

fn load_users2() -> Result<Vec<User>, UserErrors> {
    let mypath = Path::new("users.json");
    let raw_text = std::fs::read_to_string(mypath).map_err(|_| UserErrors::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UserErrors::TooManyUsers)?;
    Ok(users)
}

#[tokio::main]
async fn main() {
let _ = file_to_uppsercase();

    let my_file = Path::new("my_file.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            _ => println!("{e:#?}"),
        }
    }

    // let mut config = apis::configuration::Configuration::default();
    // config.bearer_access_token = Some("<MY_PAT>".to_string());
    // println!("{config:#?}");
    // let response = apis::control_planes_api::list_control_planes(&config, None, None, None, None, None).await;
    // match response {
    //     Ok(resp) => println!("{resp:#?}"),
    //     Err(e) => println!("{e:#?}"),
    // };
}
