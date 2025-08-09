#![allow(unused_variables)]
fn id<T>(x: T) -> T {
    x
}
pub trait Summary {
    fn summarize(&self) -> String;
}
struct States<T> {
    age: T,
    height:T,
}


impl<T: std::fmt::Display> States<T>{
    fn print_stats(
        &self
    ){
        println!("Age is {} and Height is {}",self.age, self.height);
    }
}

pub trait Largeble {
    type Output;
    fn largest(&self) -> Self::Output;
}
impl Summary for String {
        fn summarize(&self) -> String {
            format!("Summary: {}", self)
        }
    }
    
impl Largeble for Vec<f64> {
    type Output = f64;
    fn largest(&self) -> Self::Output {
        let mut largest = self[0];
        for &item in self.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
}
fn largest<T: PartialOrd + Copy>(list:&[T])->T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn notify<T:Summary>(x: T){
    println!("\n{}\n",x.summarize());
}


struct Point<T> {
    x: T,
    y: T,
    output:T,
}

impl<T: std::fmt::Display+ Copy + PartialOrd + std::ops::Add<Output =T>> Point<T>{
    fn largest_number(&self) {
        if self.x >= self.y{
            println!("X is largest number= {}", self.x);
        } else {
             println!("Y is largest number= {}", self.y);
        }
    }
    fn add(&mut self){
        println!("Adding X and Y\n");
        println!("X is \'{}\'", self.x);
        println!("Y is \'{}\'", self.y);
        println!("Output is \'{}\'", self.output);
        self.output = self.x + self.y;
        println!("Sum of X and Y is \'{}\'", self.output);
    }
}


fn main() {
    let array = [1, 2, 3, 4];
    let result = largest(&array);
    println!("The largest number is {}", result);
    let largeble_number = vec![1.0, 20.0, 3.0, 4.0];
    let result = Largeble::largest(&largeble_number);
    println!("The largest number is {}", result);
    let mut state = States{
        age: 20,
        height: 180,
    };

    state.age = 30;
    state.height;
    state.print_stats();
    let mut point = Point {
        x: 5,
        y: 10,
        output:0
    };
    Point::largest_number(&point);
    Point::add(&mut point);
    let _id = id(1);
    println!("{}",_id);
    let _id = id(1.0);
    println!("{}",_id);
    let _id = id(String::from("Ashkan"));
    println!("{}",_id);
    let ss = String::from("Ashkan");
    let summary = Summary::summarize(&ss);
    println!("{}",summary);
    notify(ss);
    let test = 12;
}
