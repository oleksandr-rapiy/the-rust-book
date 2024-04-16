fn main() {
    // let x = Some(5);

    // let y = 10;

    // match x {
    //     Some(5) => println!("x: 5"),
    //     Some(y) => println!("y: {}", y),
    //     _ => println!("None"),
    // }

    // match y {
    //     1 | 2 | 3 => println!("1, 2, or 3"),
    //     4..=10 => println!("4 to 10"),
    //     _ => println!("None"),
    // }

    let p  = Point { x: 0, y: 7 };

    let Point { x, y} = p;

}


struct  Point {
    x: i32,
    y: i32,
}