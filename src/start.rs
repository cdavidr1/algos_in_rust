fn main() {
  let hello = String::from("Hello World!");
  let borrow = &hello;
  let clone = hello.clone();
}

fn print_msg(msg: &String) {
  println!(&msg);
}
