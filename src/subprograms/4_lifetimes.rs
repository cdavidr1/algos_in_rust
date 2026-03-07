// Implement the classic longest function that takes two &str
// and returns the longer one — make it compile with explicit lifetimes,
// then break it by returning a reference to a local String
// and see the "does not live long enough" error.
//

fn main() {
    println!("{:?}", longest("abcd", "a"));
    println!("{:?}", longest_b("abcd", "a"));
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
