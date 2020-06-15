### resources...

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
