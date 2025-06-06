use futures::executor::block_on;
use futures::join;
use futures::future::join_all;

fn do_something_sync() {
    println!("Not async!");
}

async fn say_hello() {
    println!("Hello");
    join!(second_function(), say_goodbye());

    let n = double(4).await;
    println!("{n}");

    let futures = vec![double(1), double(2), double(3)];
    let results = join_all(futures).await;
    println!("{results:?}");

    do_something_sync();
}

async fn second_function()  {
    println!("Hello Again!")
}

async fn say_goodbye() {
    println!("Goodbye!")
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello())
}
