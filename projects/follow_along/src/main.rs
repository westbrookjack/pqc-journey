fn main() {
    let c: char = 'a';
    let new_c: u8 = c as u8;
    let d: char = 'A';
    let new_d: u8 = d as u8;

    let word = String::from("hello");

    println!("There are {} characters in {}.", word.chars().count(), word);
    let e: Option<char> = word.chars().nth(1);

    if let Some(ch) = e {
        println!("The first character is '{}'\n", ch);
    } else {
        println!("The string is empty.\n");
    }

    println!("The ASCII value of '{}' is {}.\n", c, new_c);
    println!("The ASCII value of '{}' is {}.\n", d, new_d);

    let x = 52-2;
    let y = x%26;
    println!("{}\n",y)
}
