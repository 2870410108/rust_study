/*泛型练习
使用一个函数可以处理多种数据类型
比如要计算返回一个动态数组里面只要是可比较的类型的最大值 */

//不使用泛型用途很单一
fn largest(list:&[i32])->&i32{
    let mut largest=&list[0];
    for item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}
//使用泛型
fn largest_generic<T:PartialOrd>(list:&[T])->&T{
    let mut largest=&list[0];
    for item in list{
        if item>largest{
            largest=item;
        }
    }
    largest
}
fn main() {
    let number_list=vec![34,50,25,100,65];
    let result=largest(&number_list);
    println!("The largest number is {}",result);

    let char_list=vec!['y','m','a','q'];
    let result=largest_generic(&char_list);
    println!("The largest char is {}",result);
}