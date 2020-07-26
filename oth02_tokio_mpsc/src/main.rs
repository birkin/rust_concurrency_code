/*
    Goal: to explore <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html#mpsc-channel>
    - An example of a multiple-producer -> single-consumer
*/

use std::thread::sleep;
use std::time;

use tokio::sync::mpsc;


async fn some_computation( input: u32 ) -> String {

    // format!( "the result of computation {}", input )

    let now = time::Instant::now();
    sleep( time::Duration::from_secs(2) );
    let msg = format!( "that_took, ``{:?}`` on thread, ``{:?}``", now.elapsed(), std::thread::current().id() ).to_string();
    println!( "msg, {:?}", msg );

    format!( "the result of computation {}", input )

}


#[tokio::main]
async fn main() {
    let ( mut tx, mut rx ) = mpsc::channel(100);

    tokio::spawn( async move {
        for i in 0..10 {
            let res = some_computation(i).await;
            tx.send( res ).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        println!( "got = {:?}", res );
    }
}
