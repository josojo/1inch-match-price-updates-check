use anyhow::Result;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootMatcha {
    pub price: String,
    pub guaranteed_price: String,
    pub to: String,
    pub data: String,
    pub value: String,
    pub gas: String,
    pub estimated_gas: String,
    pub gas_price: String,
    pub protocol_fee: String,
    pub minimum_protocol_fee: String,
    pub buy_token_address: String,
    pub sell_token_address: String,
    pub buy_amount: String,
    pub sell_amount: String,
    pub sources: Vec<Source>,
    pub orders: Vec<Order>,
    pub allowance_target: String,
    pub sell_token_to_eth_rate: String,
    pub buy_token_to_eth_rate: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub name: String,
    pub proportion: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub chain_id: i64,
    pub exchange_address: String,
    pub maker_address: String,
    pub taker_address: String,
    pub fee_recipient_address: String,
    pub sender_address: String,
    pub maker_asset_amount: String,
    pub taker_asset_amount: String,
    pub maker_fee: String,
    pub taker_fee: String,
    pub expiration_time_seconds: String,
    pub salt: String,
    pub maker_asset_data: String,
    pub taker_asset_data: String,
    pub maker_fee_asset_data: String,
    pub taker_fee_asset_data: String,
    pub signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootInch {
    pub from_token: FromToken,
    pub to_token: ToToken,
    pub to_token_amount: String,
    pub from_token_amount: String,
    pub protocols: Vec<Vec<Vec<Protocol>>>,
    pub estimated_gas: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FromToken {
    pub symbol: String,
    pub name: String,
    pub address: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToToken {
    pub symbol: String,
    pub name: String,
    pub address: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Protocol {
    pub name: String,
    pub part: i64,
    pub from_token_address: String,
    pub to_token_address: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut price_change: f64 = 0_f64;
    let mut ratio_vec = Vec::new();
    let number_of_tests: u128 = 100;
    for i in 0u128..number_of_tests {
        let request_url = format!(
            "https://api.1inch.exchange/v2.0/quote?fromTokenAddress={token}&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount={sellAmount}",
            token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            sellAmount = 10000000000000000000+i //<10 WETH>
        );
        let response = reqwest::get(&request_url).await.unwrap();
        let result: RootInch = response.json().await.unwrap();
        let old_path = &result.protocols;
        let mut string_of_protocol_names = String::from("");
        for path in old_path {
            // println!("{:?}", path);
            for split in path {
                for protocol in split {
                    string_of_protocol_names.push_str(&protocol.name);
                    string_of_protocol_names.push_str(&",");
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(3)).await;

        let request_url = format!(
            "https://api.1inch.exchange/v2.0/quote?fromTokenAddress={token}&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount={sellAmount}&protocols={protocol}",
            token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            sellAmount = 10000000000000000000+i, //<10 WETH>
            protocol= String::from("UNISWAP_V2"),
        );
        let response = reqwest::get(&request_url).await.unwrap();
        let result: RootInch = response.json().await.unwrap();
        let new_price_with_path = u128::from_str_radix(&result.to_token_amount, 10).unwrap();
        let request_url = format!(
            "https://api.1inch.exchange/v2.0/quote?fromTokenAddress={token}&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount={sellAmount}",
            token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
            sellAmount = 10000000000000000000+i, //<10 WETH>
        );
        let response = reqwest::get(&request_url).await.unwrap();
        let result: RootInch = response.json().await.unwrap();
        let new_price_without_path_restriction =
            u128::from_str_radix(&result.to_token_amount, 10).unwrap();
        println!(
            "{:?}",
            new_price_without_path_restriction as f64 / new_price_with_path as f64
        );
        price_change =
            price_change + new_price_without_path_restriction as f64 / new_price_with_path as f64;
        ratio_vec.push(new_price_without_path_restriction as f64 / new_price_with_path as f64);
    }
    println!(
        "On average the price by GP could be better than an old path by: {:?}",
        price_change / number_of_tests as f64
    );
    Ok(())
}
