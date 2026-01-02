/*
    生命周期的作用：防止发生悬垂引用
    悬垂引用：引用指向的内存已经被释放
    一般情况下，Rust的借用检查器可以防止悬垂引用的发生
    但是在某些复杂的情况下，借用检查器无法确定引用的有效性，
    这时就需要使用生命周期注解来帮助编译器进行检查
    example:
    fn func(s1:&str,s2:&str)->&str{
        if s1.len()>s2.len(){
            s1
        }else{
            s2
        }
    }
    上面的代码会报错，因为编译器无法确定返回的引用是指向s1还是s2
    解决方法：使用生命周期注解
    fn func<'a>(s1:&'a str,s2:&'a str)->&'a str{    
        if s1.len()>s2.len(){
            s1
        }else{
            s2
        }
    }
    上面的代码中，'a是一个生命周期参数，表示s1和s2的生命周期至少和'a一样长
    (换个说法就是s1和s2的生命周期不能短于'a，交叉的部分至少和'a一样长)
    这样编译器就可以确定返回的引用的有效性，避免悬垂引用的发生
    生命周期省略规则：
    1.每个引用参数都有自己的生命周期参数
    2.如果只有一个输入生命周期参数，那么该生命周期会被赋给所有输出生命周期参数
    3.如果有多个输入生命周期参数，但是其中一个是&self或
    &mut self，那么self的生命周期会被赋给所有输出生命周期参数
    example:
    fn func(s1:&str)->&str{s1}
    上面的代码是合法的，因为根据规则2
    fn func2(&self,s1:&str,s2:&str)->&str{s1}
    上面的代码是合法的，因为根据规则3
 */
fn main() {
    let s1 = String::from("hello");
    let result:&str;
    {
        let s2 = String::from("hi");
        result = func(&s1, &s2);
    }
    //尝试在string2离开作用域后使用result
    println!("The longer string is: {}", result);
    
}
fn func<'a>(s1:&'a str,s2:&'a str)->&'a str{    
        if s1.len()>s2.len(){
            s1
        }else{
            s2
        }
    }