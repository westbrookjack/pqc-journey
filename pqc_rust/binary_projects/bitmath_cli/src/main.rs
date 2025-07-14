use clap::Parser;

#[derive(Parser)] //this uses clap::Parser for this struct
#[command(name = "bitmath", about = "Performs add, sub, or xor on two integers of type i32")] //apparently I need to put this command under the above
pub struct Cli {
    operation:String,
    x:i32,
    y:i32,

    #[arg(long, help = "Outputs the result in hexadecimal")]
    hex:bool,
    #[arg(long, help = "Outputs the result in binary")]
    bin:bool,
}
fn compute(op:&str, x:i32, y:i32)->Option<i32>{
    match op {
        "add" => Some(x+y),
        "sub" => Some(x-y),
        "xor" => Some(x^y),
        _=>None,
    }
}
fn main() {
    let args = Cli::parse();
    match compute(&args.operation, args.x, args.y) {
        Some(val) => {
            if args.bin{
                println!("Result: 0b{:b}", val);
            }
            else if args.hex {
                println!("Result: 0x{:X}", val);
            }
            else {
                println!("Result: {}", val);
            }
        }
        None => eprintln!("Unsupported operation"),
    }
}


