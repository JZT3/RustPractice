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

struct BankAccount {
  owner: String,
  balance: f32,

}

// We cannot have simultaneous mutable accesss to the account
// to update the balance and immutable access for reading owner name 
impl BankAccount{
  fn withdraw(&mut self, amount: f32){
    println!("Withdrawing {} from account owned by {}", amount, self.owner);
    self.balance -= amount;
  }

  fn check_balance(&self){
    println!("Accoutn owned by {} has a balance of {}", self.owner, self.balance);
  }
}
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

  let mut account = BankAccount{
    owner: "Alice".to_string(),
    balance: 300.68
  };

  // Imutable borrow to check balance
  account.check_balance();

  // Mutable borrow to withdraw
  account.withdraw(100.25);

  account.check_balance();

}

// You can only have one mutable refernce for a single variable
// you can have many immutable referenecs for a single variable
// you cannot have both at the same time! <-- Scope enforces this
