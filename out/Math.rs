pub struct Check {
    pub a: i32,
    pub b: i32,
    pub sum: i32,
    pub product: i32,
    pub number: i32,
}

impl Check {
    pub fn main(&mut self, args: &[&str]) {
        self.a = 10;
        self.b = 5;
        self.sum = self.a + self.b;
        println!("Сумма {} и {} = {}", self.a, self.b, self.sum);
        self.product = self.a * self.b;
        println!("Произведение {} и {} = {}", self.a, self.b, self.product);
        self.number = 8;
        if self.number % 2 == 0 {
            println!("Число {} четное", self.number);
        } else {
            println!("Число {} нечетное", self.number);
        }
    }
}
