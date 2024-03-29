use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_stock_prices(tickers: Vec<String>) -> Vec<(String, f64)> {
    // Make yahoo thread-safe here.
    let yahoo = yahoo_finance_api::YahooConnector::new();
    let yahoo = Arc::new(yahoo);

    // Make result thread-safe, even with available mutability
    let mut result = Vec::with_capacity(500);
    let mut result = Arc::new(Mutex::new(result));

    // vec of futures to join later
    let mut handles = Vec::with_capacity(500);

    for ticker in tickers {
        let yahoo = yahoo.clone();
        let result = result.clone();

        let handle = tokio::spawn(async move {
            if let Ok(response) = yahoo.get_latest_quotes(&ticker, "1m").await {
                let data = (ticker, response.last_quote().unwrap().close);

                // getting control over result
                let mut result = result.lock().await;
                result.push(data);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Arc::try_unwrap(result).unwrap().into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn make_it_fly() {
        let tickers = std::fs::read_to_string("./sp500.csv")
            .unwrap()
            .lines()
            .skip(1)
            .take(500)
            .map(|line| line.split_once(',').unwrap().0.to_string())
            .collect();

        let before = chrono::Local::now();
        assert!(
            get_stock_prices(tickers).await.len() > 485,
            "Did you lower the number of fetched prices?"
        );
        let elapsed = chrono::Local::now() - before;
        assert!(
            elapsed < chrono::Duration::seconds(30),
            "Your target is to get below 30 seconds, it took: {} s",
            elapsed.num_seconds()
        );
    }
}
