use std::num::Float;

fn main() {
    let mut num = vec!(1 , 3 , 6 , 10);
    let mut a = 0us;
    let mut fac = vec![];
    for _ in (0us..1000000us) {
        let x =
            num[num.len() - 1] - num[num.len() - 2] + 1 +
                num[num.len() -
                        1];
        num.push(x);
    }
    println!("Calculating...");
    let mut _i = 1is;
    for _ in (0us..num.len() as usize) {
        for _ in (1us..(num[a] as f64).sqrt() as usize) {                                        //Logic Error
            if num[a] % _i == 0 { fac.push(_i); }
            //print!("{},\n" , res.len()); }
            _i += 1;
        }
        //some line of code here the F*cked me up for a long time
        if fac.len()*2 >= 500 {println!("Length: {}\nVector: {:?}\nValue: {}\n\nYOU GOT THE ANSWER! WOOT! \x07", fac.len(), fac, num[a]); break;}
        _i = 1;
        fac = vec![];
        a+=1
    }
}
