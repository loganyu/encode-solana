fn main() {
    println!("Welcome!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut fizz_buzz_count = 0;

    for count in 1..=301 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
    }

    println!("Number of fizz buzz occurrences: {}", fizz_buzz_count);
}
