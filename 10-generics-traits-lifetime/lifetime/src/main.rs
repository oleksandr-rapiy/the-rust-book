// fn example_of_borrow_checker() {
//                           // borrow checker
//     let a;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let b = 5;        // -+-- 'b  |
//         a = &b;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", a); //          |
// }                         // ---------+

use lifetime::CustomLongestWithAnnouncement;

fn main() {
    let str1 = String::from("1");
    let str2 = String::from("12");

    // NOTE: we tell the borrow check that
    // the lifetime of the result would be the same lifetime as
    // the smallest lifetime of the parameters value
    let result = longest(str1.as_str(), str2.as_str());
    println!("longest is {}", result);

    // let a = String::from("value_1");
    // let result: &str;
    // {
    //     let b = String::from("v 2");

    //     // NOTE: so the result lifetime would be as smallest params lifetime, means b
    //     result = longest(a.as_str(), b.as_str());
    // }
    // println!("longest is {}", result);

    use_important_excerpt();

    // NOTE: static lifetime - reference could life as a duration of the program
    let s: &'static str = "Some static lifetime ref";

    let x = String::from("value - 1");
    let y = String::from("another value 2");
    let ann = String::from("Here is a big announcement");
    let res_longest_with_ann =
        CustomLongestWithAnnouncement::longest_with_announcement(x.as_str(), y.as_str(), ann);

    println!("{}", res_longest_with_ann)
}

// NOTE:
// &i32         - a reference
// &'a i32      - a reference with an explicit lifetime
// &'a mut i32  - a mutable reference with an explicit lifetime

// NOTE: 'a - lifetime annotation
// NOTE: here we define that x, y and return will have same life time.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_v2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}

#[derive(Debug)]
struct ImportantExcerpt<'a, T> {
    part: &'a T,
}

impl<'a, T> ImportantExcerpt<'a, T> {
    fn return_part(&self, announcement: &str) -> &T {
        println!("Attention pls: {}", announcement);
        self.part
    }

    fn return_part_v2(except: &'a ImportantExcerpt<T>, announcement: &str) -> &'a T {
        println!("Attention pls: {}", announcement);
        except.part
    }
}

fn use_important_excerpt() {
    // let i;
    // {
    //     let novel = String::from("Call smt novel. Anoter tdas");
    //     let first_sentence = novel
    //         .split(".")
    //         .next()
    //         .expect("Cannt get first sentence from the novel");

    //     i = ImportantExcerpt {
    //         part: first_sentence,
    //     };
    // }

    // println!("{:?}", i.part);
}

// NOTE: lifetime elision
// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime param, that lifetime is assigned to all
//    output lifetime params

// 3. If there are multiple input lifetime params, but one of the is
//    &self or &mut self the lifetime of self is assigned to all output
//    lifetime params

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}
