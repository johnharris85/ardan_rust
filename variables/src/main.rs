fn main() {
   let mut name: String = "Hello".to_string();
   greet_borrow_mut(&mut name);
   println!("{name}");
   let input = read_line();
   println!("You typed: [{input}]")
}

fn double(n: i32) -> i32 {
    n * 2
}

fn greet_borrow_mut(s: &mut String) {
    *s = format!("{s} World!");
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}