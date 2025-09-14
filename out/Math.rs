pub struct Math
{
    pub counter: i32,
    pub message: String,
    pub flag: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color
{
    RED,
}

impl Math
{
    pub const APP: &'static str = "App";
    pub const N: i32 = 4;
    pub fn new() -> Self
    {
        Self
        {
            counter: 0,
            message: String::new(),
            flag: false,
        }
    }
    pub fn hello(who: String) -> String
    {
        return format!("hello {}", who);
    }
    pub fn sumVar(xs: &[i32]) -> i32
    {
        let mut s: i32 = 0;
        for &v in xs.iter()
        {
            s += v ;
        }
        return s;
    }
    pub fn increment(&mut self)
    {
        self.counter = self.counter + 1;
    }
    pub fn getMessage(&mut self) -> String
    {
        return format!("Счетчик: {}", self.counter);
    }
    pub fn checkEven(&mut self)
    {
        if self.counter % 2 == 0
        {
            println!("Четное: {}", self.counter);
        }
        else
        {
            println!("Нечетное: {}", self.counter);
        }
    }
    pub fn combineMessages(&mut self) -> String
    {
        return format!("{} - {}", self.message, self.counter);
    }
    pub fn main(args: &[&str])
    {
        let mut a: i32 = 10;
        let mut b: i32 = 5;
        let mut sum: i32 = a + b;
        println!("sum = {}", sum);
        for i in 0..Self::N
        {
            println!("for i = {}", i);
        }
        let mut j: i32 = 0;
        while j < 3
        {
            println!("while j = {}", j);
            j = j + 1;
        }
        let mut arr: Vec<i32> = vec![0; 3];
        let _ = 2;
        arr[(1) as usize] = 4;
        arr[(2) as usize] = 6;
        for t in 0..arr.len()
        {
            println!("arr[{}] = {}", t, arr[(t) as usize]);
        }
        let mut m = Math::new();
        m.counter = 2;
        m.message = "hello".to_string();
        println!("greet = {}", m.getMessage());
        m.checkEven();
        println!("combine = {}", m.combineMessages());
        println!("static hello bare = {}", Self::hello("neo".to_string().to_string()));
        println!("static hello via class = {}", Math.hello("neo".to_string().to_string()));
        println!("static hello via obj = {}", m.hello("neo".to_string().to_string()));
        match Color::RED
        {
            println!("{}", "enum switch RED".to_string().to_string());
            println!("{}", "enum switch GREEN".to_string().to_string());
            println!("{}", "enum switch BLUE".to_string().to_string());
        }
        println!("varargs sum(1,2,3,4) = {}", Self::sumVar(1,2,3,4));
        let mut max2: i32 = if a > b
        {
            a
        }
        else
        {
            b
        }
        ;
        println!("ternary max2 = {}", max2);
    }
}
