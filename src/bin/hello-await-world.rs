async fn say_world() {
    println!("world");
}

// $ cargo run --bin hello-await-world
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");
    println!("await ...");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}

// macro. It transforms the async fn main()
// into a synchronous fn main() that initializes
// a runtime instance and executes the async main function.
// fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//         println!("hello");
//     })
// }
