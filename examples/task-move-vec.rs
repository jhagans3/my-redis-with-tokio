use tokio::task;

// cargo run --example task-move-vec
#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    // by default, variables are not moved into async blocks.
    // without `move` the v vector remains owned
    // by the main function and the println! line borrows v.
    // `move` will instruct the compiler to move v into the spawned task.
    // Now, the task owns all of its data, making it 'static.
    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}
