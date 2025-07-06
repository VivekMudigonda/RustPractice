use std::fs;
use rand::prelude::*;
struct User {
    active: bool,
    user_name: String,
    age: u32,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn calculate_area(shape: Shape)-> f64{
    let ans = match shape {
        Shape::Circle(radius) => std::f64::consts::PI*radius*radius,
        Shape::Rectangle(width,height ) => {width*height},
        Shape::Square(side) => side*side
    };
    return ans;
}
impl Rect {
    fn area(&self) -> u32 {
        return self.width *self.height;
    }
    fn perimeter(&self) ->u32 {
        return 2*(self.width + self.height);
    }
}

fn find_first_a(s:String) -> Option<i32>{
    for(index, character) in s.chars().enumerate(){
        if character=='a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random();
    println!("Random number: {}",n);
    let res = fs::read_to_string("path");
    match res{
        Ok(content)=>{
            println!("File content: {}",content);
        }
        Err(err)=>{
            println!("Error: {}",err);
        }
    }
    let my_string = String::from("juicy");
    match find_first_a(my_string){
        Some(index) => println!("The letter a is at index {}",index),
        None => println!("The letter a is not in the string")
    }
    let name=  String::from("Alice");
    let user = User{
        user_name:name,
        age:30,
        active: true,
    };
    println!("{}",user.user_name);
    println!("{}",user.age);
    println!("{}",user.active);
    let mut x: i32 = -85;
    let y: u32 = 1000000;
    let z: f32 = 1020.0192;
    println!("x:{} y:{} z:{}",x,y,z);

    for _i   in 1..1000 {
        x = x+1000;
    }
    print!("x:{}",x);
    let is_male: bool = true;
    if is_male {
        print!("You are a male");
    }

    let greeting= "Hello world!";
    print!("{}",greeting);
    let char1 = greeting.chars().nth(1);

    match char1{
        Some(c) => println!("{}",c),
        None  => println!("There is no character at index 1000"),
    }

    let sentence: String = String::from("You are gay");
    let first_word=get_first_word(sentence);
    print!("First word is: {}", first_word);

    let circle = Shape::Circle(5.0);
    print!("Area is {}",calculate_area(circle));
    
}

fn get_first_word(sentence: String) -> String {
    let mut ans=String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

