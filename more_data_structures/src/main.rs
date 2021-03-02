fn main() {
    slices();   
    strings(); 
    tuples();
    pattern_matching();
    generics();
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);
    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a,b) = sp;
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, a, b);
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x+y, x*y)
}

fn use_slice(slice: &mut [i32])
{
    println!("first elem {} len {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1,2,3,4,5];

    use_slice(&mut data[1..4]); //taking from the second element
    println!("{:?}", data);
}

fn strings()
{
    let s:&'static str = "Hello there!";

    for c in s.chars().rev()
    {
        println!("{}", c);
    }

    let mut letters = String::new();

    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a+=1;
    }

    let z = "Hello".to_string();

    letters.push_str(&z);

    println!("{}", letters);
}

fn pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges", x, how_many(x))
    }

    let point = (3,4);

    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("y axis, x = {}", x),
        (x,y) => println!("({}, {})", x, y),
    }
}

fn how_many(x: i32) -> &'static str
{
    match x
    {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        9..=11 => "lots of oranges",
        _ if(x % 2==0) => "some",
        _ => "a few"
    }
}

fn generics()
{
    let a:Point<f64> = Point { x: 0f64, y: 4f64};
    let b = Point { x: 1.2, y: 3.4};

    let myline = Line { start: a, end:b };

    println!("{:?}", a);
    println!("{:?}", b);
}

struct Point<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}