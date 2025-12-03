use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let mut guess = String::new();
    let rand_num=rand :: thread_rng().gen_range(1..=100);
    loop {
        guess.clear();
        println!("please input your guess!");
        io::stdin().read_line(&mut guess).expect("读取失败！！");
        let guess:u32 =match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        } ;
        match guess.cmp(&rand_num) {
            Ordering::Greater=>println!("too big"),
            Ordering::Less=>println!("too small"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            } 
        }
    } 
}
