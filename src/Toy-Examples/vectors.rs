fn main(){
  let mut vec: Vec<i32> = Vec::new(); //The vector will not allocate until elements are pushed onto it.
  vec.push(1);
  vec.push(2);
  vec.push(3);

  println!("My Vector: {:?}", vec);

  let last_item = vec.pop();
  println!("My Vector: {:?}", vec);

  let first_item = vec.get(0);
  println!("My Vector: {:?}", first_item);

  let mut_item = vec.get_mut(1);


  // alternative declaration?
  /* I think this type of declaration would be used for creating a vector
  object when you know what objects should go inside or when you are manually,
  placing objects in?

  In the std Rust mentions "vec!" macro is provided for convenient initialization

  It mentions that the "vec!" macro may be more efficent than performing allocation 
  and initialization -- Particularly when initilizing a vector of zeros.

  so I assume for matrix operations the macro is preferred?
  */
  let vec2: Vec<i32> = vec![1,2,3];

  let vec3: Vec<i32> = Vec::from([1,2,3]);

  let mut vec4: Vec<i32> = Vec::with_capacity(3);
  vec4.push(2);
  vec4.push(4);
  vec4.push(6);

  let vec5: Vec<i32> = vec![3,2,1];
  vec5.shrink_to_fit();

}

/*
Since vectors are dynamic arrays you want to prevent element reallocation

Vec::with_capacity --> Constructys a new empty vector with the least specified capaciyu
without reallocating. The returned vector has the minimum capacity specified. 


*/