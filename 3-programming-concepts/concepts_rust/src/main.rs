use std::any::{Any, TypeId};
use rand::Rng;

fn main() {
    // NOTE: variables are immutable
    let mut x = 5;
    println!("The value x = {}", x);
    x = 6;

    /*
    NOTE:
    1. We can't mutate const,
    2. Const should be type annotated
    */
    const COUNT: u32 = 100;


    let t = 1;
    println!("{}", t);

    // NOTE: shadowing -> when we declare the same with same name.
    let t = 2;
    println!("{}", t);

    /*
       NOTE:
       1. Scalar data types (single value)
           integers,
           float-point,
           booleans,
           character

       2. Compound data types (group of values)
           tuple
    */

    let max_u128 = u128::MAX;
    println!("{}", max_u128);

    let max_u8 = u8::MAX;
    println!("{}", max_u8);

    // NOTE: tuple
    let tup = ("Text", 10_000);
    let (text, num) = tup;
    println!("{}", num);

    // NOTE: arrays
    let array_bytes = [0; 0];
    let last_item = match array_bytes.last() {
        Some(val) => val,
        None => &-1,
    };

    let last_item_v2 = array_bytes.last().unwrap_or(&-1);
    println!("Last array value = {}", last_item);
    println!("Last array value (v2) = {}", last_item_v2);

    /*
        NOTE: Functions
    */
    let rand_num = rand::thread_rng().gen_range(1..20);
    let result = exponentiation(rand_num);

    println!("{}^2 = {}", rand_num, result);

    let num: i64 = 100;
    print_max_type_value(&num);



    /*
        NOTE: loops (control flow)
    */
    let mut counter = 0;

    // loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break;
    //     }
    // }

    while counter != 10 {
        counter += 1;
        print!("{} ", counter);
    }

    println!();
    let array = [1, 2, 3, 4, 5, 6];
    for element in array.iter() {
        print!("{} ", element)
    }

    println!();
    for item in 1..4 {
        print!("{} ", item)
    }
}

fn exponentiation(x: i64) -> i64 {
    return i64::pow(x, 2);
}

fn print_max_type_value<T: ?Sized + Any>(value: &T) {
    let type_name = std::any::type_name::<T>();

    if is_i32(value) {
        println!("Max of {} = {}", type_name, i32::MAX);
    } else if is_i64(value) {
        println!("Max of {} = {}", type_name, i64::MAX);
    } else {
        println!("None of the implemented check!!!");
    }
}

fn is_i32<T: ?Sized + Any>(_val: &T) -> bool {
    TypeId::of::<i32>() == TypeId::of::<T>()
}

fn is_i64<T: ?Sized + Any>(_s: &T) -> bool {
    TypeId::of::<i64>() == TypeId::of::<T>()
}
