pub fn play_game(n: u32, print: bool) {
    let result = fizz_buzz_fibonacci(n);
    if print {
        println!("{result}");
    }
}
fn is_fibonacci_number(n: u32) -> bool {
    let (mut previous, mut current) = (0, 1);
    while current < n {
        let next = previous + current;
        previous = current;
        current = next;
    }
    current == n
}

pub fn fizz_buzz_fibonacci(n: u32) -> String {
    if is_fibonacci_number(n) {
        "Fibonacci".to_string()
    } else {
        match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => n.to_string(),
        }
    }
}
