fn main() {
    while_and_loop();
    if_statement();
    for_loop();
    match_statement();
}

fn if_statement() {
    let temp = 25;

    if temp > 30 {
        println!("really hot outside")
    } else if temp < 10 {
        println!("really cold")
    } else {
        println!("it's ok")
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("{}", day);

    println!(
        "is it {}",
        if temp > 20 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "Ok"
        }
    );
}

fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 {
            break;
        }
    }
}

fn for_loop()
{
    for x in 1..11
    {
        println!("{}",x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{} {}",pos,y);
    }
}

fn match_statement()
{
    let country_code = 1000;

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        351 => "PT",
        1..= 999 => "Unknown",
        _ => "invalid"
    };

    println!("the country with code {} is {}", country_code, country)
}
