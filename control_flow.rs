fn main() {
  let num = 6;

  if num % 4 == 0 {
    println!("Number is divisible by 4");
  } 
  else if num % 5 == 0 {
    println!("Number is divisible by 5");
  }
  else {
    println!("Number is not divisible by 4 or 5");
  }

  // If inside a let statement
  // If doing this each branch must be a compatible type (No data type mixing!)
  let condition = true;
  let number = if condition {5} else {6}; // Changing the 6 to "six" will throw a type error
  println!("Number: {number}");
}