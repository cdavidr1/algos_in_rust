fn main() {
    let mut v = Vec::new();
    _abc(&mut v);

    let mut swap_test = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ];
    println!("{:?}", swap_first_last(&mut swap_test));
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
