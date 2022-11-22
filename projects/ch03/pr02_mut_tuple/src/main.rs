fn main() -> () {
    let mut point = (0, 0, 0);

    dbg!(point);

    point.0 = 17;
    point.1 = 42;
    point.2 = 90;
    dbg!(point);

    let unit = ();
    dbg!(unit);

    let println_return_val = println!("Hi!");
    dbg!(println_return_val);

    set_x(point, -96);
    set_y(point,  -85);
    set_z(point, -10);
    dbg!(point);

    println!("Add all result = {}", add_all(point));
    println!("x + y result = {}", x_plus_y(point));
    println!("x + x result = {}", x_plus_x(point));
    
    
}

fn set_x(mut point: (i64, i64, i64), x: i64) {
    point.0 = x;
    dbg!(point);
}

fn set_y(mut point: (i64, i64, i64), y: i64) {
    point.1 = y;
    dbg!(point);
}

fn set_z(mut point: (i64, i64, i64), z: i64) {
    point.2 = z;
    dbg!(point);
}

fn add_all((x, y, z): (i64, i64, i64)) -> i64 {
    x + y + z
}

fn x_plus_y((x, y, _): (i64, i64, i64)) -> i64 {
    x + y
}

fn x_plus_x((x, _, _): (i64, i64, i64)) -> i64 {
    x + x
}