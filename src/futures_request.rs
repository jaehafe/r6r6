use futures::{stream, StreamExt};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();

    stream::iter(0..1000)
        .map(|i| {
            let client = client.clone();
            async move {
                println!("Hello from task {}", i);
                let resp = client.get("http://localhost:8080").send().await;
                match resp {
                    Ok(resp) => {
                        let status = resp.status();
                        let body = resp.text().await.unwrap_or_else(|_| String::new());
                        println!("Task {} got response: {} - {}", i, status, body);
                    }
                    Err(e) => println!("Task {} encountered an error: {}", i, e)
                }

            }
        })
        .buffer_unordered(100)
        .for_each(|_| async { }).await;
}