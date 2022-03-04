# Solution to the Messari challenge for computing aggregate market data from raw trades

The challenge link is [here](https://resonant-zipper-d74.notion.site/Messari-Market-Data-Coding-Challenge-rev-28-Feb-2022-e513357eaeb34b9a9ab9805af37d96b0)

The solution is written in Rust. The program takes in the piped output from executing the trade generating executable provided by Messari and computes the specified metrics for each trade as it comes in. After going through all 10 million trades generated by the executable, the program prints the computed statistics to the stdout.

## Computing metrics when a trade comes in
Each market's metrics are represented by the `MarketStats` struct. Apart from holding the stats, it also tracks the number of trades that have been generated for a particular market id. Just by keeping a track of this number, the program can easily calculate the updated stats as each trade comes in without relying on the history of all trades for each market which would be very inefficient for this challenge.

### Example: Updating mean volume when a new trade comes in from the previous mean
Since, the program knows the previous mean for a market when a trade comes in, it can easily figure out the sum of all volumes for the previous trades by `previous_mean * prev_number_of_trades`. Then it simply adds the new `volume` and divided by the updated `number_of_trades`.

Similar methods are used for calculating volume weighted mean price and buy percentage.

### To execute
Simply run the executable file in the `src` folder and pipe to `cargo run` command. This will display the stats to the stdout. This can be piped further to store the stats in a separate file as well. 

For example: `.\src\stdoutinator_amd64_windows.exe | cargo run | Out-File stats.txt -Append` (tried on windows powershell)

### Note
- After multiple checks, I have observed that for each market, the executable is generating all the `isBuy` values as true or false. Therefore, the market can have `percentage_buys` as either 1 or 0.
- Ignore the `backup.rs` file in src folder as it contains some extra code used to debug the program

