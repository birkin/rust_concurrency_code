### next

- done... just add println comments and explore.

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
