/*
    Goal: to explore <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html#mpsc-channel>
    - Copy of oth02b_tokio_mpsc_b -- trying to write to a file.
*/

use std::fs;
use std::io::prelude::*;  // needed for write!() macro
use std::thread::sleep;
use std::time;

use tokio::io;
use tokio::sync::mpsc;


#[tokio::main]
async fn main() -> io::Result<()> {
    let start_now = time::Instant::now();

    let output_filepath: String = "./output.txt".to_string();
    // -- clear output file
    fs::File::create( &output_filepath ).unwrap_or_else( |err| {
        panic!( "problem initializing the output file; error, ``{}``", err );
    });
    // -- get an append file-handler that i'll pass to the writer functions
    let fappend = fs::OpenOptions::new()
        .append(true)
        .open( &output_filepath )
        .unwrap();

    let (tx, mut rx) = mpsc::channel( 100 );
    for i in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the original handle.
        let mut tx = tx.clone();

        tokio::spawn( async move {
            let text_to_write: String = some_computation( i, start_now ).await;
            tx.send( text_to_write ).await.unwrap();
        });
    }

    // Do other work while the computation is happening in the background.
    println!( "other work can be done here" );

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.

    println!("about to call drop");
    drop( tx );
    println!("just called drop");

    while let Some( text_to_write ) = rx.recv().await {
        write!( &fappend, "\n\n{}", text_to_write ).unwrap();
        // write_to_file( &fappend, &text_to_write )
    }

    println!( "final total elapsed time, ``{:?}``", start_now.elapsed() );

    Ok( () )

}


// fn write_to_file( mut fappend: &std::fs::File, text_to_write: &str ) {
//     write!( &fappend, "\n\n{}", text_to_write ).unwrap();
// }


// -- from oth02...
async fn some_computation( input: u32, start_now: time::Instant ) -> String {

    // format!( "the result of computation {}", input )

    let now = time::Instant::now();
    sleep( time::Duration::from_secs(2) );
    let msg: String = format!( "that_took, ``{:?}`` -- for a total elapsed time of, ``{:?}`` -- on thread, ``{:?}``", now.elapsed(), start_now.elapsed(), std::thread::current().id() ).to_string();
    println!( "msg, {:?}", msg );

    let text_to_write: String = format!( "the result of computation {}", input );
    text_to_write
}
