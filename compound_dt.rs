fn main(){
  // Static Arrays
  let numbers: [i16; 5] = [1,2,3,4,5];
  println!("Number Array: {:?}", numbers);
  
  let fruits: [&str; 3] = ["Apple", "Bannana", "Orange"];
  println!("Fruits Array: {:?}", fruits);
  println!("Fruit Array 1st element: {}", fruits[0]);
  println!("Fruit Array 2nd element: {}", fruits[1]);
  println!("Fruit Array 3rd element: {}", fruits[2]);

  //Tuples
  let employees: (String, i32, bool) = ("Alice".to_string(),115000, false);
  println!("Employee Tuple: {:?}", employees);

  let mixed_tuple = ("Mario", 44, true, [1,2,3,4,5]);
  println!("Character stats: {:?}", mixed_tuple);

  // Slices
  let number_slices :&[i64] = &[1,2,3,4,5];
  println!("Number Slice: {:?}", number_slices);

  let animal_slices :&[&str] = &["Lion", "Tiger", "Jaguar"];
  println!("Number Slice: {:?}", animal_slices);

  let pet_slices :&[&String] = &[&"Cat".to_string(),
                                 &"Dog".to_string(),
                                 &"Fish".to_string()];
  println!("Number Slice: {:?}", pet_slices);

  //// Strings vs String Slices (&str) ////
  // Strings are dynamic and mutable. 
  // Strings are owned data types
  // Strings are Heap allocated

  let mut alien: String = String::from("I am, ");
  println!("Aliens from mars say: {}", alien);
  alien.push_str("a Martian!");
  println!("Aliens from mars say: {}", alien);

  // String slices are Stack allocated
  // String slices are a reference to string literals/string objects 
  // String slices are borrowed data type
  // String slices are immutable

  let my_word: String = String::from("Hello, World");
  let our_word: &str = &my_word;
  println!("String Value: {}", my_word);
  println!("Slice Value: {}", our_word);

  // Indexing 
  let our_word: &str = &my_word[0..5];
  println!("Indexed Slice Value: {}", our_word);
}