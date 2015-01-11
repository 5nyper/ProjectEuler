use std::iter::AdditiveIterator;

fn main()
{
   let (mut n, mut i, mut count, mut c, mut a) = (0i32,3i32,0i32,0i32,1i32);//int n, i = 3, count, c;
 
    n = 148933i32; 
    count = 2i32;
    let mut v = vec![2u64];
 
   if  n >= 1 
   {
      println!("Answer to question 10 is: \n");
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
        if i >= 2000000 {
          println!("done");
          break;
          return;
        }
        v.push(i as u64);
         count+=1;
      }
      i+=1;
   }
   let mut it = v.iter().map(|&x| x);
  println!("{:?}", it.sum()); 
   return;
}
