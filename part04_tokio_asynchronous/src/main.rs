use futures::stream::futures_unordered::FuturesUnordered;
use futures::stream::StreamExt;
use std::error::Error;
use std::thread::sleep;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::prelude::*;


#[tokio::main]  /* or #[tokio::main(core_threads = 1, max_threads = 1)] */
async fn main() -> Result< (), Box<dyn Error + Send + Sync> > {
    let now = Instant::now();

    // Asynchronous single-thread
    let mut futs = FuturesUnordered::new();

    futs.push( task("task1", now.clone()) );
    futs.push( task("task2", now.clone()) );
    futs.push( task("task3", now.clone()) );

    while let Some(_handled) = futs.next().await {}
    Ok( () )

}



async fn task( label: &str, now: std::time::Instant ) -> Result< (), Box<dyn Error + Send + Sync> > {
    // Simulate network delay using Tokio async delay for 2 seconds
    println!(
        "OS Thread ``{:?}`` -- ``{}`` started: ``{:?}``",
        std::thread::current().id(), label, now.elapsed(),
    );
    tokio::time::delay_for( tokio::time::Duration::from_secs(2) ).await;

    // Write to server -- server will echo this back to us with 8 second delay
    let mut stream = TcpStream::connect( "127.0.0.1:6142" ).await?;
    stream.write_all( label.as_bytes() ).await?;
    println!(
        "OS Thread ``{:?}`` -- ``{}`` written: ``{:?}`` ",
        std::thread::current().id(), label, now.elapsed()
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    stream.read_exact( &mut buffer ).await?;
    stream.shutdown( std::net::Shutdown::Both )?;
    println!(
        "OS Thread ``{:?}`` -- ``{}`` read: ``{:?}`` ",
        std::thread::current().id(), label, now.elapsed()
    );

    // Simulate computation work by sleeping actual thread for 4 seconds
    sleep( std::time::Duration::from_secs(4) );
    println!(
        "OS Thread ``{:?}`` -- ``{}`` finished: ``{:?}``",
        std::thread::current().id(), std::str::from_utf8( &buffer )?, now.elapsed()
    );

    Ok( () )

}


/* -- Output --

$
$ pwd
/path/to/concurrency_stuff/rust_concurrency_code/part04_tokio_asynchronous
$
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/part04_tokio_asynchronous`
OS Thread ``ThreadId(1)`` -- ``task1`` started: ``111.27µs``
OS Thread ``ThreadId(1)`` -- ``task2`` started: ``195.462µs``
OS Thread ``ThreadId(1)`` -- ``task3`` started: ``202.739µs``
OS Thread ``ThreadId(1)`` -- ``task3`` written: ``2.006309276s``
OS Thread ``ThreadId(1)`` -- ``task2`` written: ``2.006379328s``
OS Thread ``ThreadId(1)`` -- ``task1`` written: ``2.006434668s``
OS Thread ``ThreadId(1)`` -- ``task2`` read: ``10.007538812s``
OS Thread ``ThreadId(1)`` -- ``task2`` finished: ``14.00785164s``
OS Thread ``ThreadId(1)`` -- ``task1`` read: ``14.007996707s``
OS Thread ``ThreadId(1)`` -- ``task1`` finished: ``18.011646071s``
OS Thread ``ThreadId(1)`` -- ``task3`` read: ``18.011813749s``
OS Thread ``ThreadId(1)`` -- ``task3`` finished: ``22.016692972s``
$

*/
