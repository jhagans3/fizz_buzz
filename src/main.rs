fn main() {
    fizz_buzz();
}

struct FizzBuzz {
    multiple_of_3: bool,
    multiple_of_5: bool,
}

impl FizzBuzz {
    fn new(number: i32) -> Self {
        FizzBuzz {
            multiple_of_3: Self::multiple_of_3(number),
            multiple_of_5: Self::multiple_of_5(number),
        }
    }

    fn multiple_of_3(num: i32) -> bool {
        num % 3 == 0
    }

    fn multiple_of_5(num: i32) -> bool {
        num % 5 == 0
    }
}

fn fizz_buzz() {
    for number in 1..101 {
        match FizzBuzz::new(number) {
            FizzBuzz {
                multiple_of_3: true,
                multiple_of_5: true,
            } => println!("{} fizzbuzz", number),

            FizzBuzz {
                multiple_of_3: true,
                multiple_of_5: false,
            } => println!("{} fizz", number),

            FizzBuzz {
                multiple_of_3: false,
                multiple_of_5: true,
            } => println!("{} buzz", number),

            _ => println!("{}", number),
        }
    }
}
