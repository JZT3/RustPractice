/*
Enums offer a structured way to handle varied data under a unified type

*/

fn main() {
  enum IpAddrKind {
    V4,
    V6
  }

  let four: IpAddrKind = IpAddrKind::V4
  let six: IpAddrKind = IpAddrKind::V6

  fn route(ip_kind: IpAddrKind){}

  route(ip_kind: IpAddrKind::V4);
  route(ip_kind: IpAddrKind::V6);

  // Using Structs
  struct IpAddr{
    kind: IpAddrKind
    address: String
  }

  let home: IpAddr=
}