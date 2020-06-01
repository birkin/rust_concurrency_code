// source: <https://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en>
// his github: <https://github.com/jamesmcm/async-rust-example/tree/master/server>

use futures::stream::StreamExt;
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind( addr ).await.unwrap();

    let server = {
        async move {  // what's this `move`?
            let mut incoming = listener.incoming();
            while let Some( conn ) = incoming.next().await {
                match conn {
                    Err(e) => eprintln!( "accept failed = ``{:?}``", e ),  // whats this `eprintln!`?
                    Ok( mut sock ) => {
                        tokio::spawn( async move {
                            let ( mut reader, mut writer ) = sock.split();
                            tokio::time::delay_for( tokio::time::Duration::from_secs(8) ).await;
                            match tokio::io::copy( &mut reader, &mut writer ).await {
                                Ok( amt ) => {
                                    println!("wrote ``{:?}`` bytes", amt);
                                },
                                Err(err) => {
                                    eprintln!( "IO error, ``{}``", err );
                                }
                            }
                        });
                    }
                }

            }
        }
    };

    println!( "Server running on localhost:6142" );
    server.await;

}


/*

notes...

- From <https://doc.rust-lang.org/std/macro.eprintln.html>... "Use eprintln! only for error and progress messages. Use println! instead for the primary output of your program.

- interesting -- the `match tokio::io::copy()...` didn't require a comma after the Ok (before the Err). All the examples I'd seen before had it, so I added it and it compiles & runs, but it also compiles and runs without the comma.

- re the `async move {...`

    - don't understand that yet, but <https://doc.rust-lang.org/stable/book/ch13-01-closures.html#capturing-the-environment-with-closures> contains good info on the move keyword to "... to force the closure to take ownership of the values it uses in the environment..."

    - ah, ok. good info: <https://rust-lang.github.io/async-book/03_async_await/01_chapter.html#async-move>. Skimmed but re-read.

*/
