/*
    Goal: to explore tokio mpsc-channel
    - Another example of a multiple-producer -> single-consumer
    - Starts as copy of `oth02b_tokio_mpsc_b
*/

use std::thread::sleep;
use std::time;
use tokio::io;
use tokio::sync::mpsc;

use log;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> io::Result<()> {
    log::trace!("starting main()");

    SimpleLogger::new().init().unwrap();
    let start_now = time::Instant::now();

    let (tx, mut rx) = mpsc::channel(3);
    log::trace!("sending and receiving channels instantiated");

    for i in 0..50 {
        // Each task needs its own `tx` handle. This is done by cloning the original handle.
        let tx = tx.clone();  // std::thread::current().id() for each tx is ``ThreadId(1)``

        tokio::spawn(async move {
            // log::debug!( "thread-id, ``{:?}``", std::thread::current().id() );  // now all the thread-ids are different
            // log::debug!("in main spawn(); elapsed-a, ``{:?}``",  start_now.elapsed());
            // sleep(time::Duration::from_millis(500));
            log::debug!("in main spawn(); elapsed-a, ``{:?}``; on thread-id, ``{:?}``",  start_now.elapsed(), std::thread::current().id());
            let rslt = some_computation(i, start_now).await;
            log::debug!("in main spawn(); elapsed-b, ``{:?}``; on thread-id, ``{:?}``",  start_now.elapsed(), std::thread::current().id());
            tx.send(rslt).await.unwrap();
        });
    }

    /*  The `rx` half of the channel returns `None` once **all** `tx` clones
        drop. To ensure `None` is returned, drop the handle owned by the
        current task. If this `tx` handle is not dropped, there will always
        be a single outstanding `tx` handle.
    */
    log::trace!("about to call drop");

    drop(tx);
    log::trace!("just called drop");

    while let Some(rslt) = rx.recv().await {
        // sleep(time::Duration::from_secs(1));
        sleep(time::Duration::from_millis(500));
        log::info!("in main(); rslt, ``{:?}``", rslt);
    }

    Ok(())
}

// -- from oth02...
async fn some_computation(input: u32, start_now: time::Instant) -> String {

    let now = time::Instant::now();
    // sleep(time::Duration::from_secs(2));
    // sleep(time::Duration::from_millis(500));
    let msg = format!(
        "that_took, ``{:?}`` -- for a total elapsed time of, ``{:?}`` -- on thread, ``{:?}``",
        now.elapsed(),
        start_now.elapsed(),
        std::thread::current().id()
    )
    .to_string();
    log::debug!("in some_computation(); {:?}", msg);

    format!("input, ``{:?}``; thread-id, ``{:?}``", input, std::thread::current().id())
}
