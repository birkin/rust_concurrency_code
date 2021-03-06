/*  Grokking boxes...
    <https://doc.rust-lang.org/rust-by-example/std/box.html>

    Gist -- but other good stuff in the paragraph...
    Re `Box<T>`...
    "A box is a smart pointer to a heap allocated value of type T."  */

use std::mem;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}


/*  A Rectangle can be specified by where its top-left and bottom-right
    corners are in space  */
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}


fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}


fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new( Point {x: 0.0, y: 0.0} )
}


fn main() {
    // (all the type-annotations are superflous)

    // Stack allocated variables
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // Heap allocated rectangle
    let boxed_rectangle: Box<Rectangle> = Box::new( Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new( origin() );

    // Double indirection
    let box_in_a_box: Box< Box<Point> > = Box::new( boxed_origin() );

    println!(
        "Point occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&point) );
    println!(
        "Rectangle occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&rectangle) );

    // box size == pointer size
    println!(
        "Boxed point occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&boxed_point) );
    println!(
        "Boxed rectangle occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&boxed_rectangle) );
    println!(
        "Boxed box occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&box_in_a_box) );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies ``{:?}`` bytes on the stack",
        mem::size_of_val(&unboxed_point) );

}


/*  My thought: "Why?" -- My thinking is that the heap, though slower, is where
    you'd put something that's dynamic. Might there be other reasons? */


/*  Output...

    $
    $ cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.00s
         Running `target/debug/box_experimentation`
    Point occupies ``16`` bytes on the stack
    Rectangle occupies ``32`` bytes on the stack
    Boxed point occupies ``8`` bytes on the stack
    Boxed rectangle occupies ``8`` bytes on the stack
    Boxed box occupies ``8`` bytes on the stack
    Unboxed point occupies ``16`` bytes on the stack
    $

    */

