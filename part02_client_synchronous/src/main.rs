use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time;


fn main() -> Result< (), Box<dyn std::error::Error> > {
    let now = time::Instant::now();

    task( "task1", now.clone() )?;
    task( "task2", now.clone() )?;
    task( "task3", now.clone() )?;
    Ok( () )
}


fn task( label: &str, now: time::Instant ) -> Result< (), Box<dyn std::error::Error> > {
    // Simulate network delay using thread sleep for 2 seconds
    println!(
        "OS Thread ``{:?}`` -- ``{}`` started: ``{:?}``",
        std::thread::current().id(), label, now.elapsed()
    );
    sleep( time::Duration::from_secs(2) );

    // Write to server -- server will echo this back to us with 8-second delay
    let mut stream = TcpStream::connect( "127.0.0.1:6142" )?;
    stream.write_all( label.as_bytes() )?;
    println!(
        "OS Thread ``{:?}`` -- ``{}`` written: ``{:?}``",
        std::thread::current().id(), label, now.elapsed()
    );

    // Read 5 chars we expect (to avoid dealing with EOF, etc.)
    let mut buffer = [0; 5];
    stream.read_exact( &mut buffer )?;
    stream.shutdown( std::net::Shutdown::Both )?;
    println!(
        "OS Thread ``{:?}`` -- {} read: ``{:?}``",
        std::thread::current().id(), label, now.elapsed()
    );

    // Simulate computation work by sleeping actual thread for 4 seconds
    sleep( std::time::Duration::from_secs(4) );
    println!(
        "OS Thread ``{:?}`` -- {} finished: ``{:?}``",
        std::thread::current().id(), std::str::from_utf8( &buffer )?, now.elapsed()
    );

    Ok( () )

}


/* -- Output --

$
$ pwd
/path/to/rust_projects/concurrency_stuff/rust_concurrency_code/part02_client_synchronous
$
$ cargo run
   Compiling part02_client_synchronous v0.1.0 (/path/to/concurrency_stuff/rust_concurrency_code/part02_client_synchronous)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/part02_client_synchronous`
OS Thread ``ThreadId(1)`` -- ``task1`` started: ``694ns``
OS Thread ``ThreadId(1)`` -- ``task1`` written: ``2.005715042s``
OS Thread ``ThreadId(1)`` -- task1 read: ``10.006701192s``
OS Thread ``ThreadId(1)`` -- task1 finished: ``14.009713312s``
OS Thread ``ThreadId(1)`` -- ``task2`` started: ``14.009794504s``
OS Thread ``ThreadId(1)`` -- ``task2`` written: ``16.014866937s``
OS Thread ``ThreadId(1)`` -- task2 read: ``24.015590203s``
OS Thread ``ThreadId(1)`` -- task2 finished: ``28.015943481s``
OS Thread ``ThreadId(1)`` -- ``task3`` started: ``28.015993345s``
OS Thread ``ThreadId(1)`` -- ``task3`` written: ``30.021551602s``
OS Thread ``ThreadId(1)`` -- task3 read: ``38.026663566s``
OS Thread ``ThreadId(1)`` -- task3 finished: ``42.027353828s``
$

*/

