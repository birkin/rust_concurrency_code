use std::io::prelude::*;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Instant;

/*
    Boxing is used here, so I'm going to detour a bit and check out:
    <https://doc.rust-lang.org/rust-by-example/std/box.html>
 */

fn main() -> Result< (), Box<dyn std::error::Error> > {
    println!("Hello, world!");

    Ok( () )
}
