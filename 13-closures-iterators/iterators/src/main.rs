fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // NOTE: iterators in rust are lazy, nothing happened till we start to use
    let vec_iter = vec.iter();

    // NOTE: collect it same as .ToList() in C# to call lazy method (where, select ...)
    let list: Vec<i32> = vec_iter.map(|x| x + 1).collect();

    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 12,
            name: "das".to_string(),
        },
        Shoe {
            size: 31,
            name: "gsdfgsdf".to_string(),
        },
        Shoe {
            size: 534,
            name: "dasgdfs".to_string(),
        },
    ];

    let my_size = 12;
    for my_shoe in shoes_in_my_size(shoes, my_size) {
        println!("Name: {}, Size: {}", my_shoe.name, my_shoe.size)
    }
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    name: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    return shoes.into_iter().filter(|s| s.size == size).collect();
}

#[test]
fn iterator_demo() {
    let vec = vec![1, 2, 3, 4];

    let mut vec_iter = vec.iter();

    assert_eq!(vec_iter.next(), Some(&1));
    assert_eq!(vec_iter.next(), Some(&2));
    assert_eq!(vec_iter.next(), Some(&3));
    assert_eq!(vec_iter.next(), Some(&4));
    assert_eq!(vec_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let vec = vec![1, 2, 3, 4];

    let sum: i32 = vec.iter().sum();

    assert_eq!(sum, 10);
}

// pub trait Iterator {
//     // NOTE: associated type
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(sum, 18)
}
