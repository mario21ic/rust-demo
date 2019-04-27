fn main() {
    let mut spaces = "   ";
    println!("spaces: {}", spaces.len());

    spaces = "   xd";
    println!("spaces: {}", spaces.len());

    // This is an mismatched types error
    //spaces = spaces.len();

    let n_spaces = spaces.len();
    println!("spaces: {}", n_spaces);
}

