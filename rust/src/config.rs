use super::token::{Token, TokenPair};
use ethers::abi::Address;
use rust_decimal::Decimal;
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub uniswap_router_address: String,
    pub delay_between_checks_ms: i32,
    pub token_pairs: Vec<TokenPair>,
    pub weth_address: Address,
    pub hardcoded_redemption_value: Decimal,
    pub adapter_fee_rate: Decimal,
    pub wallet_private_key: String,
}

pub fn generate_config() -> Config {
    let weth_address: Address = env::var("WETH_ADDRESS")
        .expect("WETH_ADDRESS environment variable not set")
        .parse()
        .unwrap();

    let usdc = Token {
        symbol: String::from("USDC"),
        address: env::var("USDC_ADDRESS").expect("USDC_ADDRESS environment variable not set"),
        decimals: 6,
    };
    let usdt = Token {
        symbol: String::from("USDT"),
        address: env::var("USDT_ADDRESS").expect("USDT_ADDRESS environment variable not set"),
        decimals: 6,
    };
    let token_pairs = vec![
        // FIXME: Should we clone or make an Rc or something?
        TokenPair {
            token_in: usdc.clone(),
            token_out: usdt.clone(),
        },
        TokenPair {
            token_in: usdt,
            token_out: usdc,
        },
    ];

    Config {
        rpc_url: env::var("RPC_URL").expect("RPC_URL environment variable not set"),
        wallet_private_key: env::var("KEEPER_WALLET_PRIVATE_KEY")
            .expect("KEEPER_WALLET_PRIVATE_KEY environment variable not set"),
        uniswap_router_address: env::var("UNISWAP_ROUTER_ADDRESS")
            .expect("UNISWAP_ROUTER_ADDRESS environment variable not set"),
        delay_between_checks_ms: 3_000,
        hardcoded_redemption_value: Decimal::from(1),
        adapter_fee_rate: Decimal::from_str_exact("0.003").unwrap(),
        token_pairs,
        weth_address,
    }
}
