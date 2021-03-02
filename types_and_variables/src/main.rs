use std::mem;

const MEANING_OF_LIFE:u8 = 42;

static Z:i32 = 123;

fn main() {
    println!("{} {}",MEANING_OF_LIFE, Z);

    let a: i8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    println!(
        "c = {} after modification, size = {} bytes",
        c,
        mem::size_of_val(&c)
    );

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    c = 2;
    c += i32::pow(c, 4);
    println!("{}", c);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_cubed_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_cubed_to_pi);

    stack_and_heap()
}

struct Point{
    x:f64,
    y:f64
}

fn origin() -> Point
{
    Point {x:0.0, y:0.0}
}

fn stack_and_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("{} {}", p1.x, p2.y)
}
