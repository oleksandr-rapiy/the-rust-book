use core::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut GLOBAL_COUNTER: u32 = 0;

fn main() {
    // NOTE:
    // unsafe used to bypass the borrow checker

    // raw pointer means that it is a pointer that is not guaranteed to point to valid memory

    let mut num = 5;

    // here we are creating a raw pointer
    let r1 = &num as *const i32;

    let r2 = &mut num as *mut i32;

    unsafe {
        let t = r1.add(10);
        println!("r1: {}", *t);
    }

    println!("{}", num);

    unsafe {
        *r2 = 6;
    }

    println!("{}", num);

    unsafe {
        potentially_dangerous();
    }

    let v = vec![1, 2, 3, 4, 4];

    let ptr = v.as_ptr();

    unsafe {
        println!("ptr: {:?}", *ptr);

        println!("ptr.add(2) {:?}", *ptr.add(2));
    }

    unsafe {
        println!("abs(-3): {}", abs(-3));
    }

    println!("{}", HELLO_WORLD);

    unsafe {
        println!("GLOBAL_COUNTER: {}", GLOBAL_COUNTER);

        add_to_global_counter(10);

        println!("GLOBAL_COUNTER: {}", GLOBAL_COUNTER);
    }


    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let (first, second) = &mut v.split_at_mut(3);


    println!("{:?}, {:?}", first, second);
}

// NOTE: here we are creating a safe abstraction over unsafe code
// to call a foreign function
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn potentially_dangerous() {
    println!("This is potentially dangerous");
}

unsafe fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    )
}

unsafe fn add_to_global_counter(value: u32) {
    GLOBAL_COUNTER += value;
}
