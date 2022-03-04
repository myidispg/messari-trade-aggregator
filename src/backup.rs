use serde::{Deserialize, Serialize};
use serde_json::{Error, Result};
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

#[derive(Debug)]
struct DataPointCount {
    id: Vec<usize>,
    market: Vec<usize>,
    price: Vec<f64>,
    volume: Vec<f64>,
    is_buy: Vec<bool>,
}

#[derive(Debug)]
struct Statistics {
    total_volume: f64,
    mean_price: f64,
    mean_volume: f64,
    volume_weighted_mean_price: f64,
    percent_buy: usize,
    total_data_points: usize,
}

fn write_stats(markets: &HashMap<usize, Statistics>) {
    // Iterate over each market and write stats to stdout
    for (market_id, market_statistics) in markets.iter() {
        println!(
            "Market: {}, total volume: {}, mean price: {}, mean volume: {}, volume weighted mean price: {}, percent buy: {}",
            market_id,
            market_statistics.total_volume,
            market_statistics.mean_price,
            market_statistics.mean_volume,
            market_statistics.volume_weighted_mean_price,
            market_statistics.percent_buy
        );
        // println!("Market: {}, total volume: {}, mean price: {}, mean volume: {}, volume weighted mean price: {}, percent buy: {}, total_data_points: {}",
        //  market_id, market_statistics.total_volume, market_statistics.mean_price,
        //  market_statistics.mean_volume, market_statistics.volume_weighted_mean_price, 
        //  market_statistics.percent_buy, market_statistics.total_data_points,);
    }
}

fn main() {
    // Create a map of market id to its statistics
    let mut markets: HashMap<usize, Statistics> = HashMap::new();

    // For testing purposes to check if a duplicate market is found.
    let mut market_count: HashMap<usize, DataPointCount> = HashMap::new();

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
                        return;
                    }
                    // println!("{}", input);
                    // Parse the data.
                    let data: Result<DataPoint> = serde_json::from_str(&input);
                    match data {
                        Ok(data) => {
                            // println!(
                            //     "id: {}, market: {}, price: {}, volume: {}, is_buy: {}",
                            //     data.id, data.market, data.price, data.volume, data.is_buy
                            // );

                            // Get a mutable reference to the market if it exists or insert an new one and get a mutable reference to it.
                            let market_statistics =
                                markets.entry(data.market).or_insert(Statistics {
                                    total_volume: 0.0,
                                    mean_price: 0.0,
                                    mean_volume: 0.0,
                                    volume_weighted_mean_price: 0.0,
                                    percent_buy: 0,
                                    total_data_points: 0,
                                });

                            // Some calculations and data that will be needed while updating the statistics.
                            let is_buy: usize = if data.is_buy { 1 } else { 0 };
                            let prev_vol: f64 = market_statistics.total_volume;
                            let prev_weighted_avg: f64 =
                                market_statistics.volume_weighted_mean_price;
                            let prev_data_point_count: usize = market_statistics.total_data_points;
                            
                            /*
                            To update on the fly, new stats are calculated as follows:
                            total_volume = prev_vol + data.volume
                            mean_price = (prev_mean * prev_data_point_count + data.price) / prev_data_point_count + 1
                            mean_volume = (prev_mean * prev_data_point_count + data.volume) / prev_data_point_count + 1
                            volume_weighted_mean_price = (prev_weighted_avg * prev_vol + data.price * data.volume) / (prev_vol + data.volume)
                            percent_buy = (prev_buy_percentage * prev_data_point_count + is_buy) / prev_data_point_count + 1
                            total_data_points = prev_data_point_count + 1
                            */
                            *market_statistics = Statistics {
                                total_volume: prev_vol + data.volume,
                                mean_price: ((market_statistics.mean_price
                                    * prev_data_point_count as f64)
                                    + data.price)
                                    / (prev_data_point_count as f64 + 1.0),
                                mean_volume: ((market_statistics.mean_volume
                                    * prev_data_point_count as f64)
                                    + data.volume)
                                    / (prev_data_point_count as f64 + 1.0),
                                volume_weighted_mean_price: ((prev_weighted_avg * prev_vol)
                                    + (data.price * data.volume))
                                    / (prev_vol + data.volume),
                                percent_buy: ((market_statistics.percent_buy
                                    * prev_data_point_count)
                                    + is_buy)
                                    / (prev_data_point_count + 1),
                                total_data_points: prev_data_point_count + 1,
                            };

                            // println!("Statistics: {:?}\n", market_statistics);

                            // This will make it run until there is a duplicate market.
                            // let data_point_count =
                            //     market_count.entry(data.market).or_insert(DataPointCount {
                            //         id: vec![],
                            //         market: vec![],
                            //         price: vec![],
                            //         volume: vec![],
                            //         is_buy: vec![],
                            //     });
                            // data_point_count.id.push(data.id);
                            // data_point_count.market.push(data.market);
                            // data_point_count.price.push(data.price);
                            // data_point_count.volume.push(data.volume);
                            // data_point_count.is_buy.push(data.is_buy);

                            // if data_point_count.id.len() == 5 {
                            //     println!("Duplicate market found: {:?}", data_point_count);
                            //     return;
                            // }
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
    write_stats(&markets);
}
