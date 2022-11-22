#![allow(unused)]

fn main() {
    let point = (0,0,0);

    dbg!(point);

    println!("{} {} {}", get_x(point), get_y(point), get_z(point));
    
    {
        let x = point.0;
        let y = point.1;
        let z = point.2;
    
        println!("x y z\n{} {} {}", x, y, z);
        
    }
    {
        let (x, y, z) = point;
        println!("(x, y, z)\n({}, {}, {})", x, y, z);
    }
    
    {
        let (x, y, _) = point;
        println!("(x, y)\n({}, {})", x, y);
    }

    {
        let (x, _, _) = point;
        println!("(x)\n({})", x);
    }
}

fn get_x(my_point: (i64, i64, i64)) -> i64 {
    my_point.0
}

fn get_y(my_point: (i64, i64, i64)) -> i64 {
    my_point.1
}

fn get_z(my_point: (i64, i64, i64)) -> i64 {
    my_point.2
}