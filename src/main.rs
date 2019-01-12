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

    fn fizz_buzz_side_effect(&self, num: i32) -> () {
        println!("{} fizzbuzz", num)
    }

    fn fizz_side_effect(&self, num: i32) -> () {
        println!("{} fizz", num)
    }

    fn buzz_side_effect(&self, num: i32) -> () {
        println!("{} buzz", num)
    }
}

fn fizz_buzz() {
    for number in 1..101 {
        match FizzBuzz::new(number) {
            instance @ FizzBuzz {
                multiple_of_3: true,
                multiple_of_5: true,
            } => instance.fizz_buzz_side_effect(number),

            instance @ FizzBuzz {
                multiple_of_3: true,
                multiple_of_5: false,
            } => instance.fizz_side_effect(number),

            instance @ FizzBuzz {
                multiple_of_3: false,
                multiple_of_5: true,
            } => instance.buzz_side_effect(number),

            _ => println!("{}", number),
        }
    }
}
