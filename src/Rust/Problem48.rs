extern crate num;
use num::pow;
use num::bigint::ToBigInt;
fn main() {
   let (mut num, mut pow) = (1i64, 1us);
   let mut res = 0.to_bigint().unwrap();
   let mut slicd = "".to_string();
   loop {
      if pow == 1001 && num == 1001 {
        println!("Calculation complete!\x07");
        break;
      }
      else {
        res = res + num::pow(num.to_bigint().unwrap(), pow);
        num+=1;
        pow+=1;
      }
   }
  slicd = res.to_string();
  println!("{}", slicd.slice(slicd.len() -10, slicd.len())); 
}
