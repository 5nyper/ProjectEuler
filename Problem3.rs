//Idiomatic code
-use std::num::Float;
-
-fn is_prime(x: u64) -> bool {
-    if x < 2 { return false; }
-    if x == 2 { return true; }
-    for i in 2..((x as f64).sqrt().ceil() as u64 + 1) {
-        if x % i == 0 { return false; }
-    }
-    true
-}
-
-fn main() {
-    let mut num = 600851475143u64;
-    let mut max = 0;
-
-    let mut i = 2;
-    while i <= num {
-        if num % i == 0 && is_prime(i) {
-            println!("{} is a prime factor", i);
-            num /= i;
-            if i > max { max = i; }
-            i = 2;
-        }
-
-        i += 1;
-    }
-
-    println!("Largest prime factor: {}", max);
}
//Original Code
fn main(){
  let (mut _i,mut _j,mut _k) = (0is, 0is, 0is);
  let num = 600851475143f64;
  while _i<=num as isize{
      _k=0;
      if num%_i as f64==0f64{
         _j=1;
          while _j<=_i{
            if _i%_j==0 as isize{
                 _k += 1;
                 }
             _j+=1;
          }
          if _k==2 {
             println!("{} is a prime factor",_i);
            }
      }
      _i+=1;
   }
   return;
}
