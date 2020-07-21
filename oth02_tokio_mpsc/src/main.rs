use std::thread::sleep;
use std::time;

use tokio::sync::mpsc;


async fn some_computation( input: u32 ) -> String {

    // format!( "the result of computation {}", input )

    let now = time::Instant::now();
    sleep( time::Duration::from_secs(2) );

    let msg = format!( "that_took, ``{:?}`` on thread, ``{:?}``", now.elapsed(), std::thread::current().id() ).to_string();
    msg
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
