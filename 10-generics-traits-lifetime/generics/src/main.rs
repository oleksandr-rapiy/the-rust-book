use std::pin::Pin;

fn main() {
    let num_list = vec![1, 32, 4, 5, 5643, 4];

    let max_item = num_list.iter().max().unwrap();

    println!("Max item in array - {:?}, is {}", num_list, max_item);

    let max_item_2 = find_max(&num_list).unwrap();

    println!("Max item in array - {:?}, is {}", num_list, max_item_2);

    let str_vec: Vec<&str> = vec![];

    let max = find_max(&str_vec);

    match max {
        Some(val) => println!("Max from = {:?}, is {}", str_vec, val),
        None => println!("Unable to find max item in array"),
    }

    let p1: Point<i64, i64> = Point { x: 2, y: 3 };
    let p2_str = Point { x: "some", y: "12.3 "};

    let x = p1.x();

    let sum = p1.sum();

    println!("Sum of {:?} = {}", p1, sum);

    let mixed = p1.mix(p2_str);

    println!("Mixed Point is {:#?}", mixed);

}

// fn find_max(vec: &Vec<i32>) -> Option<i32> {
//     if vec.len() <= 1 {
//         return None;
//     }

//     let mut max = vec[0];

//     for num in vec {
//         if *num > max {
//             max = *num;
//         }
//     }

//     return Some(max);
// }

// NOTE: <T> : ... is the same as constraint in C#, but it use traits
fn find_max<T: PartialOrd + Copy>(vec: &Vec<T>) -> Option<T> {
    if vec.len() == 0 {
        return None;
    }

    let mut largest = vec.first().unwrap().to_owned();

    for num in vec {
        if *num > largest {
            largest = *num;
        }
    }
    return Some(largest);
}


#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        return &self.x;
    }

    fn y(&self) -> &U {
        return &self.y;
    }
}

impl Point<i64, i64> {
    // NOTE: the method sum will visible only for i64 type of Point struct
    fn sum(&self) -> i64 {
        self.x + self.y
    }
}

impl<T, U> Point<T, U>{
    fn mix<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}


enum CustomOption<T> {
    Some(T),
    None,
}

