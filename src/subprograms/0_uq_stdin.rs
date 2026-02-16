use std::io::{self};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    let mut times = 5;
    let mut stored = Vec::new();

    while times > 0 {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        stored.push(String::from(line.trim()));
        times -= 1;
    }

    for s in &stored {
        // get
        let mut s_count = 0;
        for a in &stored {
            if a == s {
                s_count += 1;
            }
        }

        if s_count == 1 {
            println!("Unique found: {}", s);
        }
    }
}
