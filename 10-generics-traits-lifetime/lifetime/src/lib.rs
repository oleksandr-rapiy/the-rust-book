pub mod CustomLongestWithAnnouncement {
    use std::fmt::Display;

    pub fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement - {}", ann);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
