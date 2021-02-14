
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn new() -> AveragedCollection { AveragedCollection { list: vec![], average: 0.0 } }
    pub fn show(&self) { println!("ave: {}  {:?}", self.average, self.list); }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    pub fn average(&self) -> f64 { self.average }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(10);
    ac.show();
    ac.add(15);
    ac.show();
    ac.remove();
    ac.show();
}
