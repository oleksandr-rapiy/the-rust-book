pub struct AveragedCollection {
    list: Vec<i64>,
    average: f64,
}

pub trait Averaged {
    fn average(&self) -> f64;

    fn add(&mut self, value: i64);

    fn remove(&mut self);
}

impl Averaged for AveragedCollection {
    fn average(&self) -> f64 {
        self.average
    }

    fn add(&mut self, value: i64) {
        self.list.push(value);
        self.update_average();
    }

    fn remove(&mut self) {
        self.list.pop();
        self.update_average();
    }
}

impl AveragedCollection {
    fn update_average(&mut self) {
        let sum: i64 = self.list.iter().sum();

        self.average = sum as f64 / self.list.len() as f64;
    }
}