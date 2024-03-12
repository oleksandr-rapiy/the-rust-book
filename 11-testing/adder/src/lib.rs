pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        println!("IDK can it be hold");
        self.width > other.width && self.height > other.height
    }
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Value must be more than 1");
        } else if value > 100 {
            panic!("Value must be less than 100")
        }

        Guess { value }
    }
}

// NOTE: unit test located in the same file as tested code
#[cfg(test)]
mod tests {
    // NOTE: Test is a child module, so we use 'supper' to ref to parent module
    use super::Rectangle;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn fail_test() {
    //     panic!("Failed")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 100,
            height: 100,
        };

        let smaller = Rectangle {
            width: 40,
            height: 12,
        };

        let result = larger.can_hold(&smaller);

        assert!(result);
        assert_eq!(result, true);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 1,
            height: 1,
        };

        let larger = Rectangle {
            width: 12,
            height: 12,
        };

        let result = smaller.can_hold(&larger);

        // NOTE: in Rust left doesn't mean as actual value, and right expected,
        // there are no diff between left and right assertions.
        assert_eq!(false, result);
        assert_eq!(result, false);
    }

    #[test]
    fn greeting_contain_name() {
        let name = "Carol";
        let result = greeting(name);

        assert!(
            result.contains(name),
            "{} does not contains {}",
            result,
            name
        );
    }

    #[test]
    #[should_panic] // NOTE: this attribute mean that code inside should panic!
    fn guess_new_should_panic_when_invalid_value() {
        let value = -102;

        Guess::new(value);
    }

    #[test]
    #[should_panic(expected = "Value must be less than 100")]
    fn guess_new_should_panic_when_value_more_100() {
        let value = 102;

        Guess::new(value);
    }

    #[test]
    #[ignore = "This test is pointless for nowadays"]
    fn cannot_hold_same_rectangle() {
        // assert
        let rec1 = Rectangle {
            width: 20,
            height: 20,
        };

        let rec2 = Rectangle {
            width: 20,
            height: 20,
        };

        // act
        let result = rec1.can_hold(&rec2);

        // assert
        assert_eq!(result, false);
    }
}