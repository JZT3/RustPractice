/* Structs are similar to tuples
    both hold multiple related values
    create instances of structs
*/

fn main(){
  struct Book{
    title: String,
    author: String,
    pages: u8,
    available: bool
  }

  let mut book1: Book = Book{
    title: String::from("Giovanne's Room"),
    author: String::from("James Baldwin"),
    pages: 192,
    available: true
  };

  println!("Book 1 author is {}", book1.author);

  fn write_book(author: String, title: String, pages: u8, available: bool) -> Book {
    Book{
      title,
      author,
      pages,
      available
    }
  }

  // Create instances from other instances
  let mut book2: Book = Book{
    title: String::from("Sonny's Blues"),
    ..book1
  };

  println!("Book 2 is {}", book2.title);

  // Tuple Structs
  struct Color(u8, u8, u8);
  struct Point(i32, i32, i32);

  let black: Color = Color(0,0,0);
  let white: Color = Color(255,255,255);

}
  // Unit structs
  /* 
  These are another way to implement a strucyt. They provide the basis of 
  stateless 

  They also can be used to avoid unnecessary memory allocation

  They can be used to enforce custom type safety
  */