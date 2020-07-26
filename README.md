### next

Exploring tokio sync -- <https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html>

√ make sure I can run the two existing one-shot examples
- explore the mpsc-channel examples
    x "The mpsc channel supports sending many values from many producers to a single consumer."
    x I think this is what I'd use to have multiple producers -- each reading a marc-file -- write asychronously to the output-file.
    √ try the oth02c example
    √ finish reading <http://www.randomhacks.net/2019/03/08/should-rust-channels-panic-on-send/>
    √ make a copy of oth02b and implement concurrent producers targetting a single-consumer file-writer.
    - try implementing tokio's file-writer
    - break the file-writer action out of main()

    - hmmm... maybe I don't have to write syncronously! <https://docs.rs/tokio/0.2.22/tokio/fs/struct.OpenOptions.html#method.append>



---


### resources

- <https://jamesmcm.github.io/blog/2020/05/06/a-practical-introduction-to-async-programming-in-rust/#en>
    - github code, <https://github.com/jamesmcm/async-rust-example>
    - Goes through a few different approaches.
    - Uses tokio.
    - Shows single-thread/core and multi-core approaches.

- <https://www.reddit.com/r/rust/comments/gdwuat/suckit_a_fast_multithreaded_website_downloader/>
    - "SuckIT allows you to recursively visit and download a website's content to your disk."
    - Doesn't work on Windows -- wonder what that means?

- things to check out...
    - <https://gist.github.com/lu4nm3/b8bca9431cdcf19d73040ada13387e58>
    - <http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust>
    - <https://smallcultfollowing.com/babysteps/blog/2019/12/09/async-interview-2-cramertj/>

    - [mini-redis](https://github.com/tokio-rs/mini-redis)

        - uses a semaphore to manage the max number of connections.

    - [futures_intrusive semaphore](https://docs.rs/futures-intrusive/0.3.1/futures_intrusive/sync/type.Semaphore.html)

        - no example usage there

    - [StreamExt trait](https://docs.rs/futures/0.3.5/futures/stream/trait.StreamExt.html)

        - documentation excerpts... "...the futures produced by the closure are run concurrently (but not in parallel-- this combinator does not introduce any threads)..." and "...The first argument is an optional limit on the number of concurrent futures. If this limit is not None, no more than limit futures will be run concurrently. The limit argument is of type Into<Option<usize>>, and so can be provided as either None, Some(10), or just 10. Note: a limit of zero is interpreted as no limit at all, and will have the same result as passing in None..."

        - shows 'channel' usage

        - shows an example -- try this at some point.

    - [tokio 'sync' page](https://tokio-rs.github.io/tokio/doc/tokio/sync/index.html)

        - lots of examples

        - inclues info on channels and semaphores and mutexes

        - this might be a good place to start exploration

        - another thought... go through tokio-specific tutorials.


---


### use-cases

- i have a small list of urls I want to hit.
    - so a multiple-OS-threads solution would be ok.

- i have a large list of urls I want to hit.
    - so a single or low-OS-thread solution ([green-threads](https://en.wikipedia.org/wiki/Green_threads)) would be better.

- i have a large list of urls I want to hit, but only want a maximum of 3 workers to process the list.

- i have a list of urls I want to hit, and want each worker to write results from the url-access to a file -- or intermediary storage -- in a way that won't conflict with the other workers' writes.

- i have a processor-intensive task that I'd like to split into 4 sections and run each concurrently (multi-processor).

---
