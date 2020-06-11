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

    // Asynchronous multiple-threads
    let mut futs = FuturesUnordered::new();

    futs.push( tokio::spawn( task("task1", now.clone()) ) );
    futs.push( tokio::spawn( task("task2", now.clone()) ) );
    futs.push( tokio::spawn( task("task3", now.clone()) ) );

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
/path/to/concurrency_stuff/rust_concurrency_code/part05_tokio_asynchronous_multiple_threads
$
$
$ cargo run
   Compiling part05_tokio_asynchronous_multiple_threads v0.1.0 (/path/to/concurrency_stuff/rust_concurrency_code/part05_tokio_asynchronous_multiple_threads)
    Finished dev [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/part05_tokio_asynchronous_multiple_threads`
OS Thread ``ThreadId(8)`` -- ``task1`` started: ``69.907µs``
OS Thread ``ThreadId(9)`` -- ``task2`` started: ``90.44µs``
OS Thread ``ThreadId(6)`` -- ``task3`` started: ``101.163µs``
OS Thread ``ThreadId(7)`` -- ``task2`` written: ``2.004228965s``
OS Thread ``ThreadId(6)`` -- ``task1`` written: ``2.004282031s``
OS Thread ``ThreadId(2)`` -- ``task3`` written: ``2.004219231s``
OS Thread ``ThreadId(9)`` -- ``task1`` read: ``10.004998135s``
OS Thread ``ThreadId(7)`` -- ``task3`` read: ``10.005028521s``
OS Thread ``ThreadId(2)`` -- ``task2`` read: ``10.005575235s``
OS Thread ``ThreadId(7)`` -- ``task3`` finished: ``14.008877039s``
OS Thread ``ThreadId(9)`` -- ``task1`` finished: ``14.008927367s``
OS Thread ``ThreadId(2)`` -- ``task2`` finished: ``14.008877738s``
$

*/
