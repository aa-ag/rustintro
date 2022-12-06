
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let point: Point = Point { x: 1.23, y: 4.56 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
}