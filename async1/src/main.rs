use async_std::task;

#[async_std::main]
async fn main() {
    let handle = task::spawn(async {
        println!("Hello, world!");
    });
    handle.await;
}
