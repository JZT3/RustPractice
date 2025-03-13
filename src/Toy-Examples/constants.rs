// Variables are immutable by default
// Variables != Constants

// You are not allowed to use the `mut` keyword with Constants
// You must provide types for constants. 
// Consts require all capital letters
// You can declare constants in global scope

// You can use the concept of shadowing to sanatize an immutable variable
// Shadowing should be used for local variables that are valid for short scopes
// Shadowing can be used for long-lived variables that may be passed as function parameters
// Shadowing is not the same as mutablity nor assignment
// Shadowing prevents us from having to create new variable names
//// Using mut will give you compile time error
 

fn main() {
  let mut a: u8 = 5;
  const Y: u8 = 10;

  let x = 5;     // Res = 5
  let x = x + 1; // Res = 6

  {
    let x = x * 2; // Res = 12
    println!("The value of x in inner scope {}",x);
  }

  println!("The value of x in outer scope {}", x);

  // Practical Shadowing example
  let spaces = "   ";
  let spaces = spaces.len();
  println!("num of spaces {}", spaces);
}

// One line comment

/*
This is a block comment
*/

