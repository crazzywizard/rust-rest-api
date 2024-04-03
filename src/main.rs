use reqwest::header::HeaderMap;
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiResponse {
    leaderboard_data: Vec<Leaderboard>,
    recordsCount: i32,
    totalZoraFees: String,
    totalCreatorFees: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Leaderboard {
    creator: String,
    totalCreatorReward: String,
    mainnetReward: String,
    optimismReward: String,
    baseReward: String,
    zoraReward: String,
}

async fn fetch_leaderboard_data(days: i32) -> Result<ApiResponse, Error> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    // Construct the URL with the query parameter
    let url = format!("https://api.quickindexer.xyz/leaderboard/?days={}", days);

    // Send the request to the constructed URL
    let resp = match client.get(&url).headers(headers).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(Error::from(e)),
    };

    // Deserialize the response body
    match resp.json::<ApiResponse>().await {
        Ok(resp_json) => Ok(resp_json),
        Err(e) => Err(Error::from(e)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Spawn four asynchronous tasks to fetch leaderboard data in parallel
    let task1: tokio::task::JoinHandle<Result<ApiResponse, Error>> =
        tokio::spawn(fetch_leaderboard_data(80));
    let task2: tokio::task::JoinHandle<Result<ApiResponse, Error>> =
        tokio::spawn(fetch_leaderboard_data(90));
    let task3: tokio::task::JoinHandle<Result<ApiResponse, Error>> =
        tokio::spawn(fetch_leaderboard_data(100));
    let task4: tokio::task::JoinHandle<Result<ApiResponse, Error>> =
        tokio::spawn(fetch_leaderboard_data(110));

    // Wait for all tasks to complete
    let (result1, result2, result3, result4) = tokio::try_join!(task1, task2, task3, task4,)?;

    // Process results
    println!("Result 1: {:#?}", result1?);
    println!("Result 2: {:#?}", result2?);
    println!("Result 3: {:#?}", result3?);
    println!("Result 4: {:#?}", result4?);

    Ok(())
}
