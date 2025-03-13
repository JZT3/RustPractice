// Ownership Borrowing and References
// Allows us to borrow temporarily borrow refernces to values 
// without sacrificing memory safety.

// Ownership
// ----------
// 1- Each value has an 'owner'
// 2- There can only be one owner at a time
// 3- When the owner goes out of scope, the value will be dropped.

fn calc_length(s: &String) -> usize{
  s.len()
}

fn main(){
  // ----- Principle 1 -------
  let string1 = String::from("RUST"); // Owner of this string
  let len = calc_length(&string1); // Pass by reference
  println!("Length of '{}' is {}.", string1, len);

  // ----- Principle 2 ------
  let string2 = string1;
  // println!("{}", string1); <---- This will trigger error cause it no longer owns string
  println!("{}", string2);
}


