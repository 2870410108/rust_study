use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })      
    }
}
pub fn run(config: Config)->Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("File contents: {}", contents);
    print!("Searching for {}", config.query);
    Ok(())
}