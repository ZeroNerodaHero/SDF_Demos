use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use regex::Regex;

fn get_input(msg: &str) -> Result<String, io::Error> {
    let mut ret = String::new();
    while ret.trim().is_empty() {
        print!("{}", msg);  
        io::stdout().flush()?;  
        ret.clear();
        io::stdin().read_line(&mut ret)?; 
        ret = ret.trim().to_string();
    }
    Ok(ret)
}

fn main() -> io::Result<()> {
    let conn = get_input("Name of connector: ")?;
    let mut topic_name = conn.clone();
    topic_name = get_input("Name of topic: ")?;
    let endpoint_location = get_input("Endpoint Location: ")?;
    let filename = "template.yaml";

    if !Path::new(&filename).exists() {
        println!("The template does not exist.");
        return Ok(());
    }

    let mut file = File::open(&filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents = Regex::new(r"_name_").unwrap().replace(&contents,conn.as_str()).to_string();
    contents = Regex::new(r"_topic_").unwrap().replace(&contents,topic_name.as_str()).to_string();
    contents = Regex::new(r"_endpoint_").unwrap().replace(&contents,endpoint_location.as_str()).to_string();
    println!("File contents:\n{}", contents);

    let file_path = format!("{}_connector.yaml",conn);
    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
