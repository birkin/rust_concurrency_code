/*  Grokking boxes...
    <https://doc.rust-lang.org/rust-by-example/std/box.html>  */

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
    }

    // Heap allocated rectangle
    HEREZZ
}


/*  My thought: "Why?" -- My thinking is that the heap, though slower, is where
    you'd put something that's dynamic. Might there be other reasons? */
