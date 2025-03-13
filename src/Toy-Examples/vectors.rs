fn main(){
  let mut vec: Vec<i32> = Vec::new();
  vec.push(1);
  vec.push(2);
  vec.push(3);

  println!("My Vector: {:?}", vec);

  let last_item = vec.pop();
  println!("My Vector: {:?}", vec);

  let first_item = vec.get(0);
  println!("My Vector: {:?}", first_item);

}