use std::io::{self, Write};
enum Calculator {
    Addition {x:f64,y: f64},
    Subtraction {x:f64,y: f64},
    Multiplication{x:f64,y: f64},
    Division{x:f64,y: f64},
}  
fn calculate(input:Calculator,)->f64{
    match input{
Calculator::Addition{ x, y }=> x+y,
Calculator::Subtraction{ x, y }=>x-y,
Calculator::Multiplication{ x, y }=> x*y,
Calculator::Division{ x, y }=>{
    if y != 0.0 {
        x / y
    } else {
        println!("Error: Division by zero!");
        0.0
    }
}
    }
}
fn main() {

let x=5.0;
let y=10.0;
let mut input = String::new();
io::stdout().flush().unwrap();
io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
let input = input.trim();
let operation=match input{
    "+"=>Calculator::Addition{x,y},
    "-"=>Calculator::Subtraction{x,y},
    "*"=>Calculator::Multiplication{x,y},
    "/"=>Calculator::Division{x,y},
    _ => {
        println!("Unknown operation");
        return;
    }

};
let result=calculate(operation);
println!(" the result is {}",result);

}
  
