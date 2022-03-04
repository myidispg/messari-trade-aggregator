use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io;

#[derive(Serialize, Deserialize)]
struct DataPoint {
    id: usize,
    market: usize,
    price: f64,
    volume: f64,
    is_buy: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketStats {
    market: usize,
    total_volume: f64,
    mean_price: f64,
    mean_volume: f64,
    volume_weighted_mean_price: f64,
    percent_buy: f32,
    total_data_points: usize,
}

fn write_stats(markets: &HashMap<usize, MarketStats>) -> serde_json::Result<String> {
    // Iterate over each market and write stats to stdout
    for (_, market_statistics) in markets.iter() {
        match serde_json::to_string(&market_statistics) {
            Ok(s) => println!("{}", s),
            Err(e) => return Err(e),
        }
    }

    return Ok(String::from(""));
}

fn main() {
    // Create a map of market id to its statistics
    let mut markets: HashMap<usize, MarketStats> = HashMap::new();
    // Read the input and create a representation of the data.
    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    break;
                } else {
                    // Skip the BEGIN line.
                    if input.trim() == "BEGIN" {
                        continue;
                    }
                    // Stop at the end line.
                    if input.trim() == "END" {
                        break;
                    }
                    // Parse the data.
                    let data: Result<DataPoint> = serde_json::from_str(&input);
                    match data {
                        Ok(data) => {
                            // Get a mutable reference to the market if it exists or insert an new one and get a mutable reference to it.
                            let market_statistics =
                                markets.entry(data.market).or_insert(MarketStats {
                                    market: data.market,
                                    total_volume: 0.0,
                                    mean_price: 0.0,
                                    mean_volume: 0.0,
                                    volume_weighted_mean_price: 0.0,
                                    percent_buy: 0.0,
                                    total_data_points: 0,
                                });

                            // Some data that will be needed while updating the statistics.
                            let is_buy: f32 = if data.is_buy { 1.0 } else { 0.0 };
                            let prev_mean: f64 = market_statistics.mean_price;
                            let prev_mean_volume: f64 = market_statistics.mean_volume;
                            let prev_vol: f64 = market_statistics.total_volume;
                            let prev_weighted_avg: f64 =
                                market_statistics.volume_weighted_mean_price;
                            let prev_data_point_count: usize = market_statistics.total_data_points;
                            let prev_buy_percentage: f32 = market_statistics.percent_buy;

                            /*
                            To update on the fly, old stats are used so that the program
                            does not need to recalculate the whole thing and store every data point.
                            Only the number of data points for each market is required.
                            */
                            *market_statistics = MarketStats {
                                market: data.market,
                                total_volume: prev_vol + data.volume,
                                mean_price: ((prev_mean * prev_data_point_count as f64)
                                    + data.price)
                                    / (prev_data_point_count as f64 + 1.0),
                                mean_volume: ((prev_mean_volume * prev_data_point_count as f64)
                                    + data.volume)
                                    / (prev_data_point_count as f64 + 1.0),
                                volume_weighted_mean_price: ((prev_weighted_avg * prev_vol)
                                    + (data.price * data.volume))
                                    / (prev_vol + data.volume),
                                percent_buy: ((prev_buy_percentage * prev_data_point_count as f32)
                                    + is_buy)
                                    / (prev_data_point_count as f32 + 1.0),
                                total_data_points: prev_data_point_count + 1,
                            };
                        }
                        Err(e) => {
                            println!("Error while parsing json: {}", e);
                            return;
                        }
                    }
                }
            }
            Err(error) => {
                println!("Error while reading the input stream: {}", error);
                return;
            }
        }
    }
    // Write to stdout all the stats
    match write_stats(&markets) {
        Ok(_) => {}
        Err(e) => println!("Error while writing to stdout: {}", e),
    }
}
