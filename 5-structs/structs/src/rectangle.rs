pub mod rectangle {
    // NOTE: provide basic imp of Debug
    #[derive(Debug)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        // NOTE: methods
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.width > other.height 
        }
    }

    impl Rectangle {

        // NOTE: associated function, not a method.
        pub fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size
            }
        }
    }
}
