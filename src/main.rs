use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Photo {
    albumId: i32,
    id: i32,
    title: String,
    url: String,
    thumbnailUrl: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://jsonplaceholder.typicode.com/photos");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let photos: Vec<Photo> = response.json().await?;
    println!("{:?}", photos);
    Ok(())
}
