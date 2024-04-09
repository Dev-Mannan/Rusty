
use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]


async fn main()->Result<()>{

    let res = reqwest::get("https://reqres.in/api/users/2").await?;
    println!("Status : {}",res.status());
    println!("Headers:\n {:#?}",res.headers());
    let body = res.text().await?;
    println!("\nBody: \n{}",body);
    Ok(())

}