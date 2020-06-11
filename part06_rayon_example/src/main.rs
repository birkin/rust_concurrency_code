use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time;

use rayon::prelude::*;



fn main() -> Result< (), Box<dyn std::error::Error + Send + Sync> > {
    let now = time::Instant::now();

    ["task1", "task2", "task3"]
        .par_iter()
        .map( |x| task(x, now.clone()) )
        .collect::< Result<Vec<_>, _> >()?;

    Ok( () )
}


fn task( label: &str, now: std::time::Instant ) -> Result< (), Box<dyn std::error::Error + Send + Sync> > {
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
/path/to/rust_projects/concurrency_stuff/rust_concurrency_code/part06_rayon_example
$
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/part06_rayon_example`
OS Thread ``ThreadId(2)`` -- ``task1`` started: ``356.326µs``
OS Thread ``ThreadId(4)`` -- ``task2`` started: ``363.248µs``
OS Thread ``ThreadId(9)`` -- ``task3`` started: ``365.414µs``
OS Thread ``ThreadId(9)`` -- ``task3`` written: ``2.003421296s``
OS Thread ``ThreadId(4)`` -- ``task2`` written: ``2.003446765s``
OS Thread ``ThreadId(2)`` -- ``task1`` written: ``2.003422499s``
OS Thread ``ThreadId(9)`` -- task3 read: ``10.00707891s``
OS Thread ``ThreadId(4)`` -- task2 read: ``10.007126565s``
OS Thread ``ThreadId(2)`` -- task1 read: ``10.007087274s``
OS Thread ``ThreadId(9)`` -- task3 finished: ``14.007976242s``
OS Thread ``ThreadId(4)`` -- task2 finished: ``14.007990658s``
OS Thread ``ThreadId(2)`` -- task1 finished: ``14.007996274s``
$

*/

