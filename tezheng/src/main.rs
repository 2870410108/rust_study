//特征基础练习
//定义一个可以打印自身的方法
pub trait Summary {
    fn summarize(&self) -> String;//定义一个方法签名,没有默认实现
    fn default_summarize(&self) -> String {//定义一个带有默认实现的方法
        String::from("(Read more...)")
    }
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,   
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    //重写默认实现的方法
    fn default_summarize(&self) -> String {
        format!("(Read more from {}...)", self.author)
    }
}
#[derive(Debug)]//为结构体实现Debug特征
#[allow(dead_code)]//避免未使用警告
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
//impl Summary for Tweet {} //使用默认实现

fn display_summary(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}
fn main() {
    let news_article = NewsArticle {
        headline: String::from("Rust is awesome!"),
        location: String::from("Internet"),
        author: String::from("Rustacean"),
    };
    display_summary(&news_article);
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Check out Rust!"),
        reply: false,
        retweet: false,
    };
    //无法编译通过，因为Tweet没有实现Summary特征
    //display_summary(&tweet);
    print!("{:?}", tweet);
}