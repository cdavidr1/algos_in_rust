use std::time::Instant;

fn main() {
    let mut v = Vec::new();
    _abc(&mut v);

    let mut swap_test = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];
    println!("{:?}", swap_first_last(&mut swap_test));

    let words = ["abc", "five", "three"];
    println!("{:?}", longest_ref(&words));

    const ITERATIONS: usize = 1_000_000;

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        trim_and_append_world(String::from("hello"), true);
    }
    let d1 = start.elapsed();
    println!("{:?}", d1);
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        trim_and_append_world(String::from("hello"), false);
    }
    let d2 = start.elapsed();
    println!("{:?}", d2);
}

fn _abc(a: &mut Vec<i32>) {
    for i in 0..=10 {
        a.push(i);
    }
    borrow_abc(a);
}
fn borrow_abc(i: &Vec<i32>) {
    println!("{}", i.len());
}

// swaps the first and last elements without taking ownership or cloning the strings
fn swap_first_last(v: &mut Vec<String>) -> &Vec<String> {
    let last_index = v.len() - 1;
    v.swap(0, last_index);
    v
}

// return ref to longest string in a slice
fn longest_ref<'a>(s: &'a [&str]) -> &'a str {
    let mut x = "";
    for &st in s {
        if st.len() > x.len() {
            x = st;
        }
    }
    x
}

// trim option benchmark next time
fn trim_and_append_world(mut s: String, o: bool) -> String {
    s = s.trim().to_string();
    if s == "hello" {
        s = s.to_uppercase();
    }

    if o {
        s = s + " world!";
    } else {
        s.push_str(" world!");
    }
    s
}
