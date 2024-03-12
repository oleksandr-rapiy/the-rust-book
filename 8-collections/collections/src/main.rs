use std::{collections::{hash_map, HashSet}, io::{self, Write}};
use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    // v.push(2);
    println!("third element is {}", third);

    match v.get(20) {
        Some(value) => println!("{}", value),
        None => println!("Invalid value"),
    }

    let value = v.get(20).unwrap_or(&-1);

    println!("{}", value);

    let mut v_2 = vec![1, 2, 3, 4, 5, 6];
    for i in &mut v_2 {
        // NOTE: de-reference operator
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(32.2),
        SpreadsheetCell::Text(String::from("Some data")),
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not an int")
    }

    

    // NOTE: strings 
    // String are stored as a collection of UTF-8 encodes bytes

    let s_1 = String::new();
    let s_2 = "str";
    let s_3 = s_2.to_string();

    let mut main_str = String::from("foo");
    main_str.push_str("s");

    let s_4 = s_1 + &s_3;


    // for b in main_str.bytes() {
    //     println!("{}", b);
    // }


    // NOTE: grapheme chars
    for g in "sad".graphemes(true) {
        println!("{}", g)   
    }


    // NOTE: hash maps

    let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 12);
    // scores.insert(yellow, 13);

    println!("{:#?}", scores);

    let score = scores.get(&String::from("Blue"));


    scores.entry(String::from("Yellow")).or_insert(10);

    for (key, value) in &scores {
        println!("{} - {}", key, value);   
    }


    let text = "this is some funny string this";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


    // NOTE: HashSet


    let mut hash_set_collection: HashSet<String> = HashSet::new();

    hash_set_collection.insert(String::from("some"));
    hash_set_collection.insert(String::from("some 1"));
    hash_set_collection.insert(String::from("some 2"));

    let contains_some_1 = hash_set_collection.contains(&String::from("some 1"));

    println!("hash set contains - 'some 1' = {}", contains_some_1);

}



