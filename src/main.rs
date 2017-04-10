use std::io;

fn main() {
    println!("What is your first name?");

    let mut f_name = String::new();

    io::stdin().read_line(&mut f_name)
        .expect("Failed to read line");

    println!("Your first name is {}. Is this correct?", f_name);
    println!("y/n");


}

fn answer() {
    let mut ans = String::new();

    io::stdin().read_line(&mut ans)
        .expect("Failed to read line")
}