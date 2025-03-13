// Rust allows Hoisting of functions, 
// but I would prefer to declare them above main

fn talk(){
  println!("Hello, Rust!");
}

// Input values
fn human_height(height: u8){
  println!("Your height is {} cm", height);
}

fn id_card(name: &str, age: u8, height: f32){
  println!("My name is {}, I am {} years old, and I am {} inches tall", name, age, height);
}

fn add(a: i64, b: i64) -> i64{
  a + b // Statement
}

fn calc_bmi(weight_kg: f64, height_m: f64) -> f64{
  weight_kg / (height_m * height_m)
}

fn main() {
  talk();
  human_height(182);
  
  let name :&str = "Bob";
  let age :u8 = 71;
  let height :f32 = 174.0;
  id_card(name, age, height);

  let product: i128 = {
    let price: i128    = 5;
    let quantity: i128 = 10;
    price * quantity // Statement
  };

  println!("The product price is {}", product);

  let y: i64 = add(40, 60);
  println!("The value of y is : {}", y);

  println!("The value of 'add' function is {}.",   add(4, 6));

  let weight_kg: f64 = 70.9;
  let height_m : f64 = 1.68;
  let bmi      : f64 = calc_bmi(weight_kg, height_m);
  println!("BMI is: {:.2}", bmi); // We limited to display only 2 decimal points
}

// Expressions: Anything that returns a value
//// numbers
//// booleans
//// evaluation
//// if conditions
//// curly braces
//-------------------
// Statement: Anything that does not return a value
// Typically end in a ;
// You cannot assign two statements to each other
// variable declaration
// function defintiions
// c