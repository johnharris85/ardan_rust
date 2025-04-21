use authentication::{LoginRole, User, get_users, save_users};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
    /// Add a user
    Add {
        /// User login name
        username: String,
        /// User password (plaintext)
        password: String,
        /// Optional - mark as admin
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user
    Delete {
        /// Username to delete
        username: String,
    },
    /// Change user password
    ChangePassword {
        /// User's password to change
        username: String,
        /// New password 
        password: String,
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role");
    println!("{:-<40}", "");

    let users = authentication::get_users();
    users
        .iter()
        .for_each(|(_, user)| println!("{:<20}{:<20?}", user.username, user.role))
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users = get_users();
    let role = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let user = User::new(&username, &password, role);
    users.insert(username, user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("{username} does not exist!");
    }
}

fn change_password(username: String, password: String) {
    let mut users = get_users();
    if let Some(user) = users.get_mut(&username) {
        user.password = authentication::hash_password(&password);
        save_users(users);
    } else {
        println!("{username} does not exist!")
    };
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Some(Commands::List) => {
            list_users();
        }
        Some(Commands::Add {
            username,
            password,
            admin,
        }) => {
            add_user(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete { username }) => {
            delete_user(username);
        }
        Some(Commands::ChangePassword { username, password }) => {
            change_password(username, password);
        }
        None => {
            println!("Run with --help for docs");
        }
    }
}
