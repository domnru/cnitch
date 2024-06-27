use reqwest::Response;

#[tokio::main]
async fn main() {
    use std::env;
    let url: String = env::var("CNITCH_URL").unwrap_or("http://localhost:8080".to_owned());
    let mut args: env::Args = env::args();
    let app_name: String = args.nth(0).unwrap_or("Unknown".to_owned());
    let params: String = Vec::from_iter(args).join(" ");

    // I won't expose a possible error to the attacker
    if let Err(_) = snitch(format!("{} {}", app_name, params), url).await { }
}

async fn snitch(name: String, url: String) -> Result<Response, reqwest::Error> {
    use reqwest;

    let client = reqwest::Client::new();
    let res = client.post(url)
        .body(name)
        .send()
        .await;

    return res;
}