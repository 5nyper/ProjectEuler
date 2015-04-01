//Fast code
fn main() {
    let mut i = 20is;
    loop {
        if i % 19 == 0 && i % 18 == 0 && i % 17 == 0 && i % 16 == 0 && i % 15 == 0 && i % 14 == 0
            && i % 13 == 0 && i % 12 == 0 && i % 11 == 0 {
                print!("result: {}", i);
                break;
            }
        i += 20;
    }
}
