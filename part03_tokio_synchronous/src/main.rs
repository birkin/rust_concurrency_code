use futures::stream::StreamExt;
use std::error::Error;
use std::thread::sleep;
use std::time::Instant;
use tokio::join;
use tokio::net::TcpStream;
use tokio::prelude::*;


#[tokio::main]
async fn main() -> Result< (), Box<dyn Error + Send + Sync> > {
    let now = Instant::now();

    //Synchronous
    task( "task1", now.clone() ).await?;
    task( "task2", now.clone() ).await?;
    task( "task3", now.clone() ).await?;
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
/path/to/rust_projects/concurrency_stuff/rust_concurrency_code/part03_tokio_synchronous
$
$ cargo run
warning: unused import: `futures::stream::StreamExt`
 --> src/main.rs:1:5
  |
1 | use futures::stream::StreamExt;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `tokio::join`
 --> src/main.rs:5:5
  |
5 | use tokio::join;
  |     ^^^^^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/part03_tokio_synchronous`
OS Thread ``ThreadId(1)`` -- ``task1`` started: ``2.676µs``
OS Thread ``ThreadId(1)`` -- ``task1`` written: ``2.004426857s``
OS Thread ``ThreadId(1)`` -- ``task1`` read: ``10.010604972s``
OS Thread ``ThreadId(1)`` -- ``task1`` finished: ``14.015766721s``
OS Thread ``ThreadId(1)`` -- ``task2`` started: ``14.015987529s``
OS Thread ``ThreadId(1)`` -- ``task2`` written: ``16.020298699s``
OS Thread ``ThreadId(1)`` -- ``task2`` read: ``24.026515453s``
OS Thread ``ThreadId(1)`` -- ``task2`` finished: ``28.031672588s``
OS Thread ``ThreadId(1)`` -- ``task3`` started: ``28.031810998s``
OS Thread ``ThreadId(1)`` -- ``task3`` written: ``30.036323133s``
OS Thread ``ThreadId(1)`` -- ``task3`` read: ``38.042498357s``
OS Thread ``ThreadId(1)`` -- ``task3`` finished: ``42.04751145s``
$

*/
