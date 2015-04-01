extern crate num;
use num::pow;
use num::bigint::ToBigInt;
fn main() {
   let (mut num, mut pow) = (2i64, 2us);
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
  li.dedup();
  println!("{}", li.len()); 
}
