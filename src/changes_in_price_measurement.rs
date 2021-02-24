use anyhow::Result;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

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
    pub maker_token: String,
    pub taker_token: String,
    pub maker_amount: String,
    pub taker_amount: String,
    pub fill_data: FillData,
    pub source: String,
    pub source_path_id: String,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillData {
    pub token_address_path: Option<Vec<String>>,
    pub router: Option<String>,
    pub pool_address: Option<String>,
    pub network_address: Option<String>,
    pub path: Option<Vec<String>>,
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
    let mut price_change: u32 = 0;
    let mut previous_result_price = 0 as u128;
    let start = Instant::now();
    for i in 0..50 {
        let new_matcha_price: u128 = get_matcha_price(i).await.unwrap();
        let new_1inch_price: u128 = get_1inch_price(i).await.unwrap();
        let new_price = u128::max(new_1inch_price, new_matcha_price);
        let price_diff = previous_result_price as f64 / new_price as f64;
        if i != 0 {
            if price_diff < 0.99999 as f64 || price_diff > 1.00001 as f64 {
                println!(
                    "New price diff of {:?} with old price of {:?} and new price of {:?}",
                    price_diff, previous_result_price, new_price
                );
                price_change += 1;
            }
        }
        // if new_matcha_price > new_1inch_price {
        println!("Best price from matcha {:?}", new_matcha_price);
        // } else {
        println!("Best price from 1inch {:?}", new_1inch_price);
        // }
        previous_result_price = new_price.clone();

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    let duration = start.elapsed();
    println!(
        "On average the price changed by {:?} / {:?}",
        price_change, duration
    );
    Ok(())
}

async fn get_matcha_price(i: u128) -> Result<u128> {
    let request_url = format!(
        "https://api.0x.org/swap/v1/quote?buyToken=DAI&sellToken={token}&sellAmount={sellAmount}",
        token = "WETH",
        sellAmount = (100000000000000000000 + i) //<100 WETH>
    );
    let response = reqwest::get(&request_url).await?;
    let result: RootMatcha = response.json().await?;
    let number = u128::from_str_radix(&result.buy_amount, 10)?;
    Ok(number)
}
async fn get_1inch_price(i: u128) -> Result<u128> {
    let request_url = format!(
        "https://api.1inch.exchange/v2.0/quote?fromTokenAddress={token}&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount={sellAmount}",
        token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        sellAmount = 100000000000000000000+i //<100 WETH>
    );
    let response = reqwest::get(&request_url).await?;
    let result: RootInch = response.json().await?;
    let number = u128::from_str_radix(&result.to_token_amount, 10)?;
    Ok(number)
}
