use std::env;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, TransactionParameters, H160, U256};

#[tokio::main]


async fn main() -> web3::Result<()> {
    println!("Hello, world!");
    dotenv::dotenv().ok();
    // connecting to an RPC node to query the blockchain
    let http_transport = web3::transports::Http::new(&env::var("HTTP_INFURA").unwrap())?;

    let websockets = web3::transports::WebSocket::new(&env::var("GOERLI_INFURA").unwrap()).await?;
    let web3 = web3::Web3::new(websockets);
    println!("http: {:?}", web3);

    // tell the chain I'm looking for accounts in this vector
    let mut accounts = web3.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("accounts in: {:?}", accounts);

    let wei_conversion: U256 = U256::exp10(18);
   for account in &accounts {
        let balance = web3.eth().balance(*account , None).await?;
        println!("Eth Balance for {:?}: is {:?}", account, balance.checked_div(wei_conversion).unwrap() );
    }

    // Query the swap router contract for Uniswapv3 on Goerli 
    let uniswap_router_addr = Address::from_str("0xE592427A0AEce92De3Edee1F18E0157C05861564").unwrap();
    let uniswap_router_contract = Contract::from_json(web3.eth(), uniswap_router_addr, include_bytes!("uniswap_abi.json")).unwrap();
    // Note this requires a type definition in order to unwrap;
    let test: Address  = uniswap_router_contract
        .query("WETH9", (), None, Options::default(), None).await.unwrap();
      


    let uniswap_factory = Address::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap();
    let factory_contract = Contract::from_json(web3.eth(), uniswap_factory, include_bytes!("uniswapV3Factory_abi.json")).unwrap();
    // WETH9 token on Goerli
    //0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6
    let weth_address = Address::from_str("0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6").unwrap();
    // Uniswap Token to swap for
    let uni_address = Address::from_str("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984").unwrap();
    let dai_address = Address::from_str("0x4687afccbe4ec31d1f2dcfc5d207f6b1ef1b4c7e").unwrap();

    let recipient_address = Address::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap();
    // Uniswap ERC20 on Goerli
    // 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984
    // need to get pool of the address

    let test2: Address = factory_contract.query("getPool", (weth_address, dai_address, 3000 as u32), None, Options::default(), None)
        .await
        .unwrap();

    // need to call get pool function
    println!("pool address: {:?}", test2);

    // pool address may not be necessary as can input token address 
    struct ExactInputSingleParams {
        tokenIn: Address,
        tokenOut: Address,
        fee: u32,
        recipient: Address,
        deadline: u128,
        amountIn: u128,
        amountOutMinimum: u128,
        sqrtPriceLimitX96: u128
    }
    let input_params = ExactInputSingleParams {
        tokenIn : weth_address,
        tokenOut: uni_address,
        fee: 3000,
        recipient: recipient_address,
        deadline: 0 as u128,
        amountIn: 0 as u128, 
        amountOutMinimum: 0 as u128,
        sqrtPriceLimitX96: 0 as u128,
    };
    let deadline = get_valid_timestamp(300000);
    // Estimate Gas function
    let test4 = uniswap_router_contract.abi();
    println!("abi: {:?}", test4);
    let gas_estimate = uniswap_router_contract.estimate_gas("exactInputSingle",
    ((weth_address,
        dai_address,
        3000 as u128,
        recipient_address,
       deadline,
       0 as u128,
        0 as u128 ,
        0 as u128) )  ,
     accounts[0],
     Options::default())
    .await
    .unwrap();

    println!("estimated gas: {:?}", gas_estimate);
   //U256::from_dec_str(&deadline.to_string()).unwrap(),
    //U256::from_dec_str("10000000000000000").unwrap()  ,
    //U256::from_dec_str("106662000000").unwrap()
    //let signed_transaction = web3.accounts().sign_transaction(input_params, key);
    Ok(())
}

pub fn get_valid_timestamp(future_millis: u128) -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();
    
    time_millis
}
