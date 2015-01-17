extern crate num;
use num::pow;
use num::bigint::ToBigInt;
fn main() {
   let (mut num, mut pow) = (2i64, 2us);
   let (mut a, mut b) = (0us, 1us);
   let mut li = vec![];
   loop {
      if pow == 101 && num == 100 {
        println!("Math is done! Ordering them out...\x07");
        break;
      }
      else if pow == 101 {
        pow = 2us;
        num+=1;
      }
      li.push(num::pow(num.to_bigint().unwrap(), pow));
      pow+=1;
   }
  li.sort();
  for _ in (0..li.len() - 1) {
        if li[a] == li[b] {
            li.remove(b);
        } else {
            a+=1;
            b+=1;
        }
    }
  println!("{:?}", li.len()); 
}
