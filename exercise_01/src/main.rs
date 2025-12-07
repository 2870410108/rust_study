//使用每一种方式计算一个矩形面积
// fn main() {
//     let width = 10;
//     let height = 13;
//     println!("this rectangle area is {}", area(width, height));
// }
// fn area(width:u32,height:u32)->u32{
//     width*height
// }

// fn main() {
//     let rect = (56, 13);
//     println!("this rectangle area is {}", area(rect));
// }
// fn area(rect:(u32,u32))->u32{
//     rect.0 * rect.1
// }
// 
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect{
    fn area(&self)->u32{
        self.width * self.height
    }
    fn is_legal(&self)->bool{
        self.width>0 && self.height>0
    }
}
use std::io;
fn main(){
    let mut buf=String::new();
    loop {
        buf.clear();
        println!("please input width and height of rectangle:");
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let width = buf.trim().parse().expect("Please type a number!");
        buf.clear();
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        let height = buf.trim().parse().expect("Please type a number!");
        let rect= Rect{
        width,
        height,
        };
        if rect.is_legal()==false{
        println!("this rectangle is not legal");
        continue;
        }
        dbg!(&rect);
        println!("this rectangle area is {}",rect.area());
    }
}