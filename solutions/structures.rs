#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: Rectangle) -> f32 {
    let height: f32 = r.bottom_right.x - r.top_left.x;
    let width: f32 = r.bottom_right.y - r.top_left.y;
    return height.abs() * width.abs();
}

fn square(p: &Point, side: f32) -> Rectangle {
    return Rectangle {
        top_left: Point {x: p.x, y: p.y},
        bottom_right: Point {x: p.x + side, y: p.y + side}
    };
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Print rectangle Area
    let my_rectangle: Rectangle = Rectangle {
        top_left: Point {x: 0.0, y: 0.0},
        bottom_right: Point {x: 5.0, y: 5.0}
    };
    println!("rectangle area {:.2}", rect_area(my_rectangle));

    // Print square
    let my_point: Point = Point { x: 0.0, y: 0.0};
    let my_rectangle2 = square(&my_point, 6.0);
    println!("point coordinates: ({}, {})", my_point.x, my_point.y);
    println!("square area {:.2}", rect_area(my_rectangle2));
}