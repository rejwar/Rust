use reqwest;

#[tokio::main]
async fn main() {
    println!("Data is fetched ");
    let fetched_data = fetch_data().await;

    match fetched_data {
        Ok(data) => {
            println!("Data is fetched successfully {}", data);
        }
        Err(e) =>{
            eprintln!("data fetch is probleming {}", e);
        }
    }
}

async fn fetch_data() -> Result< String, reqwest::Error> {
    let url = "kjga;dkjfh";
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)


}