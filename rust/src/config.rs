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
    // FIXME: Move to a config.rs file or similar?
    let weth_address: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .unwrap();
    let usdc = Token {
        symbol: String::from("USDC"),
        // FIXME: Convert to Address instances?
        address: String::from("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
        decimals: 6,
    };
    let usdt = Token {
        symbol: String::from("USDT"),
        address: String::from("0xdac17f958d2ee523a2206206994597c13d831ec7"),
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
        // FIXME: Replace with env var
        uniswap_router_address: String::from("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"), // WETH
        delay_between_checks_ms: 3_000,
        hardcoded_redemption_value: Decimal::from(1),
        adapter_fee_rate: Decimal::from_str_exact("0.003").unwrap(),
        token_pairs,
        weth_address,
    }
}
