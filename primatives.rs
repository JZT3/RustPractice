fn main(){
  let x: i8 = -42;
  let y: u16 = 100;

  println!("signed int: {}", x);
  println!("unsigned int: {}", y);

  let pi: f32 = 3.14;
  let e:  f64 = 2.72;

  println!("float 32: {}", pi);
  println!("float 64: {}", e);

  let is_sunny: bool = true;
  println!("Is it sunny outside? {}", is_sunny);

  let letter: char = 'a';
  println!("First letter of the alphabet: {}", letter);
}