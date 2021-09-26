use std::future::Future;
use futures::{executor};

fn some_great_function() -> impl Future<Output = i32> {
    async {
        let value: i32 = 5;
        send_to_another_thread_with_borrowing(&value).await
    }
}

async fn send_to_another_thread_with_borrowing(x: &i32) -> i32 {
    println!("{}", x);
    *x
}

fn main() {
    executor::block_on(some_great_function());
}
