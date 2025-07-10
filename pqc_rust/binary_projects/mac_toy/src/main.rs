mod mac;

use mac::PrefixMac;

fn main() {
    let key = [0u8; 32];
    let msg = b"hello world";

    let mac = PrefixMac::new(&key);
    let tag = mac.compute(msg);

    println!("Tag: {:x?}", tag);

    let valid = mac.verify(msg, &tag);
    println!("Valid tag? {}", valid);
}
