fn main() {
    println!("Hello, world!");

    let answer = do_twice(add_one, 5);

    let answer2 = do_twice_generic(add_one, 5);

    let statuses: Vec<Status> = (0..10).map(Status::Value).collect();

    println!("statuses: {:?}", statuses);
}

fn add_one(x: i32) -> i32 {
    return x + 1;
}

fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    return f(x) + f(x);
}

fn do_twice_generic<F>(f: F, ags: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    return f(ags) + f(ags);
}

// We define the function which take as args function and args,
// and return the other function which return the T
fn return_closure<F, T>(f: F, arg: T) -> impl Fn(T) -> T
where
    F: Fn(T) -> T,
    T: Human,
{
    return |x| x;
}

fn closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a < 10 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}

trait Human {}

trait God: Human {}

#[derive(Debug)]
enum Status {
    Value(u32),
}
