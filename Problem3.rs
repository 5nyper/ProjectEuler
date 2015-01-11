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
