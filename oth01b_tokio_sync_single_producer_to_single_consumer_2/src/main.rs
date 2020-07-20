/*
    Goal: to explore <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html#oneshot-channel>
        - specifically the `JoinHandle` approach that's an alternative to one-channel.
    Another example of a single-producer -> single-consumer
*/

use std::thread::sleep;
use std::time;


#[tokio::main]
async fn main() {
    let start = time::Instant::now();

    let join_handle = tokio::spawn( async move {
        some_computation().await
    } );
    // let zz: () = join_handle;  // yields: found struct `tokio::runtime::task::join::JoinHandle`

    let join_handle_2 = tokio::spawn( async move {
        some_computation().await
    } );

    // Do other work while the computation is happening in the background.
    println!( "other work can be done here" );

    // Wait for the computation result
    let res = join_handle.await.unwrap();
    println!( "res, ``{:?}``", res );

    let res2 = join_handle_2.await.unwrap();
    println!( "res2, ``{:?}``", res2 );

    println!( "whole thing took, ``{:?}``", start.elapsed() );
}


async fn some_computation() -> String {
    // "the result of the computation".to_string()
    let now = time::Instant::now();

    sleep( time::Duration::from_secs(2) );

    let return_msg = format!( "that_took, ``{:?}`` on thread, ``{:?}``", now.elapsed(), std::thread::current().id() ).to_string();
    return_msg
}
