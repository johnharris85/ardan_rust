use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}


pub fn greet_user(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

// pub fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "password", LoginRole::Admin),
//         User::new("bob", "password", LoginRole::User),
//     ]
// }

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        // Load file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        // Create a file and return
        let users = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "password", LoginRole::Admin),
    );
    users.insert(
        "bob".to_string(),
        User::new("bob", "password", LoginRole::User),
    );
    users
}

// pub fn get_admin_users() {
//     let users: Vec<String> = get_users()
//     .into_iter()
//     .filter(|u| u.role == LoginRole::Admin)
//     .map(|u| u.username)
//     .collect();
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let password = hash_password(password);
    let users = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    // if let Some(user) = users.iter().find(|user| user.username == username) {
    //     if user.password == password {
    //         return Some(LoginAction::Granted(user.role.clone()));
    //     } else {
    //         return Some(LoginAction::Denied);
    //     }
    // }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello John!", greet_user("John"));
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(login("admin1", "password"), None);
        assert_eq!(
            login("Bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("Bob", "password1"), Some(LoginAction::Denied));
    }
}
