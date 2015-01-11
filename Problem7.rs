//Idiomatic code
-use std::num::Float;
-
-fn is_prime(x: i32) -> bool {
-    if x < 2 { return false; }
-    if x == 2 { return true; }
-    for i in 2..((x as f64).sqrt().ceil() as i32 + 1) {
-        if x % i == 0 { return false; }
-    }
-    true
-}
-
-fn main() {
-    const N: i32 = 10_001;
-    let mut count = 2i32;
-
-    if  N >= 1 {
-        println!("First {} prime numbers are:", N);
-        println!("1: 2");
-    }
-
-    let mut i = 3i32;
-    while count <= N {
-        if is_prime(i) {
-            println!("{}: {}", count, i);
-            count += 1;
-        }
-        i += 1;
-    }
}
//Original Code
fn main()
{
   let (mut n, mut i, mut count, mut c, mut a) = (0i32,3i32,0i32,0i32,1i32);//int n, i = 3, count, c;
 
    n = 10001i32;
    count = 2i32;
 
   if  n >= 1 
   {
      println!("First {} prime numbers are :\n",n);
      println!("1: 2\n");
   }
 
   while count <= n //( count = 2 ; count <= n ;  )
   {
       c = 2i32;
      while c <= i // OR for c in range(c, i -1) ( c = 2 ; c <= i - 1 ; c++ )
      {
         if  i%c == 0  {
            break;
            }
        c+=1
      }
      if  c == i 
      {
        a+=1;
         println!("{}: {}\n",a,i,);
         count+=1;
      }
      i+=1;
   }
 
   return;
}
