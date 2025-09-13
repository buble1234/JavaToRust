pub struct Math {
    pub counter: i32,
    pub message: String,
    pub flag: bool,
}

impl Math {
    pub fn increment(&mut self) {
        self.counter = self.counter + 1;
    }
    pub fn getMessage(&self) -> String {
        return format!("Счетчик: {}", self.counter);
    }
    pub fn checkEven(&self) {
        if self.counter % 2 == 0 {
            println!("Четное число: {}", self.counter);
        } else {
            println!("Нечетное число: {}", self.counter);
        }
    }
    pub fn combineMessages(&self) -> String {
        return format!("{} - {}", self.message, self.counter);
    }
}
