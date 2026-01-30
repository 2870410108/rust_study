fn main() {
    // 修复方案：将类型指定为字符串切片 &str
    embed_resource::compile("icon.rc", &[] as &[&str]);
}
