use futures::executor;

fn print_result(value: i32) {
    println!("{}", value);
}

async fn async_add(left: i32, right: i32) -> i32 {
    left+ right
}

async fn calculate() -> i32 {
    let add1 = async_add(2, 3).await;
    print_result(add1);
    let add2 = async_add(3, 4).await;
    print_result(add2);
    async_add(add1, add2).await
}

fn main() {
    executor::block_on(calculate());
}
