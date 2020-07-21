/*
    Goal: to explore <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html#oneshot-channel>
    Example of a single-producer -> single-consumer
*/

use std::thread::sleep;
use std::time;
use tokio::sync::oneshot;


// #[tokio::main]
/*
    or #[tokio::main(core_threads = INT, max_threads = INT)] -- eg #[tokio::main(core_threads = 4, max_threads = 8)]
    <https://docs.rs/tokio/0.2.21/tokio/runtime/struct.Builder.html#method.core_threads>
*/

#[tokio::main]
async fn main() {
    let start = time::Instant::now();

    let (tx, rx) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    tokio::spawn(async move {
        let res2 = some_computation().await;
        tx2.send(res2).unwrap();
    });

    // Do other work while the computation is happening in the background
    println!( "other work can be done here" );

    // Wait for the computation result
    let res = rx.await.unwrap();
    println!( "res, ``{:?}``", res );

    let res2 = rx2.await.unwrap();
    println!( "res2, ``{:?}``", res2 );

    println!( "whole thing took, ``{:?}``", start.elapsed() );
}


async fn some_computation() -> String {
    let now = time::Instant::now();

    sleep( time::Duration::from_secs(2) );

    let return_msg = format!( "that_took, ``{:?}`` on thread, ``{:?}``", now.elapsed(), std::thread::current().id() ).to_string();
    return_msg
}

// Output...

// if: #[tokio::main]
// other work can be done here
// res, ``"that_took, ``2.005126532s`` on thread, ``ThreadId(9)``"``
// res2, ``"that_took, ``2.005152004s`` on thread, ``ThreadId(8)``"``
// whole thing took, ``2.005520476s``

// if: #[tokio::main(core_threads = 1, max_threads = 1)]
//
// other work can be done here
// res, ``"that_took, ``2.005170655s`` on thread, ``ThreadId(1)``"``
// res2, ``"that_took, ``2.005142026s`` on thread, ``ThreadId(1)``"``
// whole thing took, ``4.010902085s``

// if: #[tokio::main(core_threads = 2, max_threads = 1)]
// error: max_threads cannot be less than core_threads

// if: #[tokio::main(core_threads = 10, max_threads = 10)]
// res, ``"that_took, ``2.005138424s`` on thread, ``ThreadId(11)``"``
// res2, ``"that_took, ``2.005167158s`` on thread, ``ThreadId(10)``"``
// whole thing took, ``2.005574863s``
// (the ThreadId will always be 10 or 11)

// if: #[tokio::main(core_threads = 4, max_threads = 8)]
// other work can be done here
// res, ``"that_took, ``2.003758865s`` on thread, ``ThreadId(5)``"``
// res2, ``"that_took, ``2.003749324s`` on thread, ``ThreadId(3)``"``
// whole thing took, ``2.00405526s``
// birkin@bbox-2015$
// (one of the ThreadIds will always be 5, the other, in testing, was 2 or 3 or 4)
