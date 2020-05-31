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
                    Ok( HEREZZ )
                }

            }
        }
    };

}
