use std::mem;

fn main() {
    structures();
    enums();
    option();
    arrays();
    vectors();
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("point p is at {}, {}", p.x, p.y);
    let myline = Line { start: p, end: p2 };
    println!("my line start is {}", myline.start.x);
}

fn enums() {
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 0,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("{},{},{}", r, g, b),
        _ => (),
    }
}

fn option() {
    // Option<T>

    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);

    match result {
        Some(z) => println!("{}", z),
        None => println!("cannot divide {} by {}", x, y),
    }

    if let Some(z) = result {
        println!("z = {}", z);
    }
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    a[0] = 321;

    println!("a has {} elements, first is {}", a.len(), a[0]);

    println!("{:?}", a);

    let b = [1u8; 10];

    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("{},{}: {}", i, j, mtx[i][j])
            };
        }
    }
}

fn vectors() {
    let mut a = Vec::new();
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    match a.get(6) {
        Some(x) => println!("{}", x),
        None => println!("No value."),
    }

    for x in &a {
        println!("x = {}", x);
    }
    let last_elem = a.pop();

    println!("last elem {:?}", last_elem);
}
