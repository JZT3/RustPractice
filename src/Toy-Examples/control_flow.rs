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

  // Loops
  let mut counter = 0;
  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {result}");

  // Loop Lables
  let rows = 3;
  let cols = 3;
  let mut matrix = vec![vec![0; cols]; rows];

  'outer: for i in 0..rows {
    'inner: for j in 0..cols {
      matrix[i][j] = i * cols + j;
      println!("matrix[{}][{}] = {}", i,j, matrix[i][j]);
    }
  }

  println!("{:?}", matrix);


  // While loop
  let mut thing = 3;
  while thing != 0 {
    println!("{number}");
    thing -= 1;
  }
  println!("BANG!!");

  // Loop through collections
  let a = [1,2,3,4,5];

  for element in a {
    println!("{element}")
  }
}