// References
// Safety 
// -- prevention of memeory bugs (undefined behavior)
////// null pointer de-referenceing
////// dangling pointers
////// buffer overflow
////// data races

/// You create referenes by borrowing from the original owner of a value (Principle 2)

//// Mutable References -- Borrowing without modification + modify data
// Create Reference by using the "&"

impl BankAccount{
fn main(){
  // Immutable Referenes 
  let x:u8 = 5;
  // let r: u8 = x; <-- this type of assignment is bad and x variable ceases to exist
  let r: &u8 = &x;

  println!("Value of x: {}", x);
  println!("Value of r: {}", r);

  // *r += 1; <-- This wont work because r is an immutable reference

  let mut y:i8 = 1;
  let p: &mut i8 = &mut y; 

  *p += 1;
  println!("Value of p: {}", p);
  *p -= 5;
  println!("Value of p: {}", p);

  println!("Value of y: {}", y);
}

// You can only have one mutable refernce for a single variable
// you can have many immutable referenecs for a single variable
// you cannot have both at the same time! <-- Scope enforces this
