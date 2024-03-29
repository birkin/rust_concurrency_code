/*
    Goal: to explore <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html#mpsc-channel>
    - Another example of a multiple-producer -> single-consumer
*/

use std::thread::sleep;
use std::time;
use tokio::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let start_now = time::Instant::now();
    println!("start_now, ``{:?}``", start_now );

    let (tx, mut rx) = mpsc::channel(100);

    for i in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the original handle.
        let mut tx = tx.clone();

        tokio::spawn(async move {
            let res = some_computation(i, start_now).await;
            tx.send(res).await.unwrap();
        });
    }

    /*  The `rx` half of the channel returns `None` once **all** `tx` clones
        drop. To ensure `None` is returned, drop the handle owned by the
        current task. If this `tx` handle is not dropped, there will always
        be a single outstanding `tx` handle.
    */
    println!("about to call drop");
    drop(tx);
    println!("just called drop");

    while let Some(res) = rx.recv().await {
        // socket.write_all(res).await?;
        sleep(time::Duration::from_secs(1));
        println!("res, ``{:?}``", res);
    }

    Ok(())
}

// -- from oth02...
async fn some_computation(input: u32, start_now: time::Instant) -> String {
    // format!( "the result of computation {}", input )

    let now = time::Instant::now();
    sleep(time::Duration::from_secs(2));
    let msg = format!(
        "that_took, ``{:?}`` -- for a total elapsed time of, ``{:?}`` -- on thread, ``{:?}``",
        now.elapsed(),
        start_now.elapsed(),
        std::thread::current().id()
    )
    .to_string();
    println!("msg, {:?}", msg);

    format!("the result of computation {}", input)
}
