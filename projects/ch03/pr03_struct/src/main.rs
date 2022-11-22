#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn main() {
    let point = Point { x: 1, y: 2, z: 3 };
    dbg!(&point);

    let x = point.x;
    let Point { x, y, z } = &point;

    let point = new_point(1, 2, 3);
    dbg!(&point);

    let Point { x, y: _, z } = &point;
    let Point { x, z, .. } = &point;
    let Point { x, .. } = &point;

    let mut point = Point { x: 1, y: 2, z: 3 };
    dbg!(&point);
    point.x = 5;

    dbg!(&point);
}

fn new_point(x: i64, y: i64, z: i64) -> Point {
    Point { x: x, y: y, z: z }
}

fn get_x(point: Point) -> i64 {
    point.x
}

fn set_x(mut point: Point, x: i64) {
    point.x = x;
}

fn x_plus_y(point :Point) -> i64 {
    let Point { x, y, .. } = point;
    x + y
}


