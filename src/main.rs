use std::env;
use std::marker::StructuralEq;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

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
    for account in accounts {
        let balance = web3.eth().balance(account , None).await?;
        println!("Eth Balance for {:?}: is {:?}", account, balance.checked_div(wei_conversion).unwrap() );
    }

    // Query the swap router contract for Uniswapv3 on Goerli 
    let weth_addr = Address::from_str("0xE592427A0AEce92De3Edee1F18E0157C05861564").unwrap();
    let token_contract = Contract::from_json(web3.eth(), weth_addr, include_bytes!("uniswap_abi.json")).unwrap();
    // Note this requires a type definition in order to unwrap;
    let test: Address  = token_contract
        .query("WETH9", (), None, Options::default(), None).await.unwrap();
      


    let uniswap_factory = Address::from_str("0x1F98431c8aD98523631AE4a59f267346ea31F984").unwrap();
    let factory_contract = Contract::from_json(web3.eth(), uniswap_factory, include_bytes!("uniswapV3Factory_abi.json")).unwrap();
    // WETH9 token on Goerli
    //0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6
    let weth_address = Address::from_str("0xB4FBF271143F4FBf7B91A5ded31805e42b2208d6").unwrap();
    let uni_address = Address::from_str("0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984").unwrap();
    println!("factory: {:?}", factory_contract);

    // Uniswap ERC20 on Goerli
    // 0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984
    // need to get pool of the address

    let test2: Address = factory_contract.query("getPool", (weth_address, uni_address, 3000 as u32), None, Options::default(), None)
        .await
        .unwrap();

    // need to call get pool function
    println!("pool address: {:?}", test2);

    // pool address may not be necessary as can input token address 

    let input_parmas = {
        "tokenIn" : weth_address,
        
    }

    Ok(())
}
