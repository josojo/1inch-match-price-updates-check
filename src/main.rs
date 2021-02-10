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
    let mut price_change: u32 = 0;
    let mut previous_result_price = 0 as u128;
    let start = Instant::now();
    for i in 0..50 {
        let new_matcha_price: u128 = get_matcha_price().await.unwrap();
        let new_1inch_price: u128 = get_1inch_price().await.unwrap();
        if i != 0 {
            if previous_result_price != u128::max(new_matcha_price, new_1inch_price) {
                price_change += 1;
            }
        }
        // if new_matcha_price > new_1inch_price {
        println!("Best price from matcha {:?}", new_matcha_price);
        // } else {
        println!("Best price from 1inch {:?}", new_1inch_price);
        // }
        previous_result_price = u128::max(new_matcha_price, new_1inch_price).clone();

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    let duration = start.elapsed();
    println!(
        "On average the price changed by {:?} / {:?}",
        price_change, duration
    );
    Ok(())
}

async fn get_matcha_price() -> Result<u128> {
    let request_url = format!(
        "https://api.0x.org/swap/v1/quote?buyToken=DAI&sellToken={token}&sellAmount={sellAmount}",
        token = "WETH",
        sellAmount = "100000000000000000" //<100 WETH>
    );
    let response = reqwest::get(&request_url).await?;
    let result: RootMatcha = response.json().await?;
    let number = u128::from_str_radix(&result.buy_amount, 10)?;
    Ok(number)
}
async fn get_1inch_price() -> Result<u128> {
    let request_url = format!(
        "https://api.1inch.exchange/v2.0/quote?fromTokenAddress={token}&toTokenAddress=0x6b175474e89094c44da98b954eedeac495271d0f&amount={sellAmount}",
        token = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
        sellAmount = "100000000000000000" //<100 WETH>
    );
    let response = reqwest::get(&request_url).await?;
    let result: RootInch = response.json().await?;
    let number = u128::from_str_radix(&result.to_token_amount, 10)?;
    Ok(number)
}
