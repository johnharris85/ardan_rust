use authentication::{LoginAction, login, read_line};

fn main() {
    let mut tries: i32 = 5;
    loop {
        if tries <= 0 {
            println!("FAILED!");
            break;
        }
        println!("Enter your username: ");
        let username: String = read_line();
        println!("Enter your password: ");
        let password = read_line();
        match login(&username, &password) {
            // Multiple ways to match, match is very powerful
            //LoginAction::Granted(authentication::LoginRole::Admin) => println!("Admin"),
            Some(LoginAction::Granted(role)) => {
                match role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            Some(LoginAction::Denied) => {
                // Nothing
            }
            None => {
                println!("New User System")
            }
        }

        tries -= 1;
        println!("Nope! Only {tries} tries left!");
    }
}
