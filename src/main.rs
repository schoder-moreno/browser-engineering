use std::env;

use http::HeaderMap;
use reqwest::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let url = reqwest::Url::parse(&args[1]).unwrap();

    load(url).await;
}

async fn load(url: Url) {
    let response = request(url).await;
    match response {
        Ok(content) => show(content.0),
        Err(e) => println!("{e}")
    }
}

fn show(content: String) {
    let mut current_tag = String::new();
    let mut current_entity = String::new();

    let mut in_angle = false;
    let mut in_body = false;
    let mut in_entity = false;

    for char in content.chars(){
        if char == '<' {
            in_angle = true;
            current_tag.clear();
        } else if char == '>' {
            in_angle = false;
            if current_tag.eq("body") {
                in_body = true;
            } else if current_tag.eq("/body") {
                in_body = false;
            }
        } else if char == '&' {
            in_entity = true;
            current_entity.clear();
        } else if char == ';' && in_entity {
            in_entity = false;
            if current_entity.eq("lt") {
                print!("<");
            } else if current_entity.eq("gt") {
                print!(">");
            } 
        } else if in_entity && !in_angle {
            current_entity.push(char);
        } else if in_angle {
            current_tag.push(char);
        } else if in_body {
            print!("{char}");
        }
    }
}

async fn request(url: Url) -> Result<(String, HeaderMap), reqwest::Error>{
    let client = reqwest::Client::new();
    
    let res = client.get(url).send().await?;
    let headers = res.headers().to_owned(); 
    let body = res.text().await?;

    return Ok((body, headers))
}