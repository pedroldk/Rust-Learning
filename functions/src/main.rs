fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn functions() {
    print_value(5);

    let mut z = 1;
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);

    println!("p = {}", p);
}

fn main() {
    functions();
    methods();
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("point p is at {}, {}", p.x, p.y);

    let myline = Line { start: p, end: p2 };
    println!("my line start is {}", myline.start.x);

    println!("The length is {}", myline.len());
}