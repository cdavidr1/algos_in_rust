// Implement the classic longest function that takes two &str
// and returns the longer one — make it compile with explicit lifetimes,
// then break it by returning a reference to a local String
// and see the "does not live long enough" error.
//

fn main() {
    println!("{:?}", longest("abcd", "a"));
    // println!("{:?}", longest_b("abcd", "a"));
    //
    //

    let mut h = Holder::new("abc");
    println!("{:?}", h.get());
    h.extends("kek");
    println!("{:?}", h.get());
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    s2
}

// fn longest_b<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     let local = if s1.len() > s2.len() {
//         s1.to_string()
//     } else {
//         s2.to_string()
//     };
//     &local
// }

// Create a struct Holder<'a> { data: &'a str } with a method
//  get(&self) -> &'a str and another extend(&mut self, extra: &'a str)
// that appends to an internal String (you'll need to fight mutability + lifetime conflicts).
//

struct Holder {
    data: String,
    // data: &'a str,
}

impl Holder {
    fn new(data: &str) -> Self {
        Holder {
            data: data.to_string(),
        }
    }
    // fn get(&self) -> &'a str {
    //     self.data
    // }
    fn get(&self) -> &String {
        &self.data
    }
    fn extends(&mut self, extra: &str) {
        self.data = self.data.to_string() + extra;
    }
}
