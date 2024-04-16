use std::{collections::HashMap, thread, time::Duration};

fn main() {
    // NOTE: closures - anonymous functions

    generate_workout(12, 321);

    // NOTE: we can to not define input and output type
    // as compiler can determinate types, after first invocation of the closure.

    let closure_x = |x| x;
    let t = closure_x(4);

    // NOTE: this will not work 'cause above we call closure with i32 type
    // let t = closure_x(String::from("This will not work"));

    let x = vec![1, 2, 3, 4];
    

    // NOTE: 

    // there three types of closures
    // Fn - immutably borrows values 
    // FnOnce - take ownership over the var and can be called once 
    // FnMut - mutably borrows values 


    // NOTE: move means x closure takes ownership over x
    let equal_to_x = move |num| num == x;

    // println!("vec = {:?}", x);

    let y = vec![1, 23, 4];

    assert!(equal_to_x(y));
}

// NOTE: memoization
struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::hash::Hash + Eq + Clone,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: std::hash::Hash + Eq + Clone,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, args: U) -> U {
        match self.value.get(&args) {
            Some(value) => value.clone(),
            None => {
                let v = (self.calculation)(args.clone());
                self.value.insert(args, v.clone());
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, rand_number: i32) {
    let mut cache = Cacher::new(|num| {
        println!("Calculating ...");
        thread::sleep(Duration::from_secs(2));
        return num * 2;
    });

    println!("Result {}", cache.value(12));
    println!("Result {}", cache.value(13));
    println!("Result {}", cache.value(1));
    println!("Result {}", cache.value(12));

    // if intensity < 24 {
    //     println!("Today, do {} pushups!", cache.value(intensity));
    //     println!(
    //         "Next, do {} pushups!",
    //         cache.value(u32::try_from(rand_number).unwrap_or(0))
    //     )
    // }
}
