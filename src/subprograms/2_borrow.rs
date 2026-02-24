fn main() {
    let mut v = Vec::new();
    _abc(&mut v);
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
