// source: <https://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en>
// his github: <https://github.com/jamesmcm/async-rust-example/tree/master/server>

use futures::stream::StreamExt;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
