fn main() {
    let lang = Language::Japanese;

    match lang {
        Language::English => println!("Hello"),
        Language::Ukrainian => println!("Привіт"),
        Language::Spanish => println!("Hola"),
        Language::Japanese => println!("こんにちは"),
        lang => println!("I don't know {:?}", lang),
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[derive(Debug)]
enum Language {
    English,
    Ukrainian,
    Spanish,
    Japanese,
} 