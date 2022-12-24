use eyre::Result;
use std::sync::Arc;
use ethers::prelude::*;
use std::convert::TryFrom;

abigen!(
    IUniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

abigen!(
    IDirt,
    r#"[
        function balanceOf(address to, uint256 tokenId) external view returns (uint256)
    ]"#,
);
pub async fn get_price(address: String) -> Result<ethers::types::U256> {
    let client = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/ef5de06be90e40c29efc35534f46a5dd",
    )?;
    let client = Arc::new(client);
    let parsed_address = address.parse::<Address>()?;
    let dirt_address = "0x73d70C603fd639Fc10AEb58af8a646b2Defd34c5".parse::<Address>()?;
    // ETH/USDT pair on Uniswap V2
    // let address = "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".parse::<Address>()?;
    // let pair = IUniswapV2Pair::new(address, Arc::clone(&client));
    let dirt = IDirt::new(dirt_address, Arc::clone(&client));
    let token_id = ethers::types::U256::from(2);
    let balance = dirt.balance_of(parsed_address, token_id).call().await?;
    // dirt.
    // // getReserves -> get_reserves
    // let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
    // println!("Reserves (ETH, USDT): ({reserve0}, {reserve1})");

    // let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
    // println!("ETH/USDT price: {mid_price:.2}");
    Ok(balance)
}
