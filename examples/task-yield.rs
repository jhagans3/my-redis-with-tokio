use std::rc::Rc;
use tokio::task::yield_now;

// cargo run --example task-yield
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope forces `rc` to drop before `.await`.
        {
            let rc = Rc::new("hello");
            println!("{}", rc);
        }

        // `rc` is no longer used. It is **not** persisted when
        // the task yields to the scheduler
        yield_now().await;
    });
}

// When .await is called, the task yields back to the scheduler.
// The next time the task is executed, it resumes from the point
// it last yielded. To make this work, all state that is used after
// `.await` must be saved by the task.
// If this state is Send, it can be moved across threads
// then the task itself can be moved across threads.
// Conversely, if the state is not Send, then neither is the task.
/*
async fn main() {
    tokio::spawn(async {
        // the trait
        // `std::marker::Send` is not  implemented for
        // `std::rc::Rc<&str>`
        let rc = Rc::new("hello");

        // `rc` is used after `.await`. It must be persisted to
        // the task's state.
        yield_now().await;

        println!("{}", rc);
    });
}
*/
