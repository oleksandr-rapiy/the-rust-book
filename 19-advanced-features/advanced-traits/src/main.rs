use std::{fmt::Debug, ops::Add};

fn main() {
    let poor_country = PoorCountry {
        population: 1000,
        gdp: 1000.0,
    };

    let amount = poor_country.summarize();

    poor_country.outline_print();

    println!("Amount: {}", amount);

    let point1 = Point { x: 1, y: 2 };

    let point2 = Point { x: 4, y: 6 };

    let sum = point1 + point2;

    println!("Sum of points equals: ({:?})", sum);

    let human = Human;



    // Pilot::fly(&human);

    // Wizard::fly(&human);

    <Human as Wizard>::fly(&human);
    <Human as Pilot>::fly(&human);
}

pub trait Summary {
    type Amount: Debug;

    fn summarize(&self) -> Self::Amount;
}

pub struct PoorCountry {
    population: u32,
    gdp: f64,
}

impl Summary for PoorCountry {
    type Amount = f64;

    fn summarize(&self) -> f64 {
        self.gdp / self.population as f64
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, point: Self) -> Self::Output {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Human now can fly");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Human now is Harry Potter");
    }
}

trait OutlinePrint: Summary {
    fn outline_print(&self) {
        println!("{:?}", self.summarize());
    }
}


impl OutlinePrint for PoorCountry {

}
