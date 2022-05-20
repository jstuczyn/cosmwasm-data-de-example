use common::{ExecuteMsg, InstantiateMsg};
use cosmrs::proto;
use prost::Message;
use std::io::Read;
use validator_client::nymd::cosmwasm_client::{connect_with_signer, signing_client};
use validator_client::nymd::wallet::DirectSecp256k1HdWallet;
use validator_client::nymd::{AccountId, CosmWasmClient, GasPrice, SigningCosmWasmClient};

async fn upload_and_initialise(
    client: &signing_client::Client,
    address: &AccountId,
) -> anyhow::Result<AccountId> {
    let contract_path = "target/wasm32-unknown-unknown/release/dummy_contract.wasm";
    let mut file = std::fs::File::open(contract_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("uploading the smart contract...");
    let upload_res = client
        .upload(
            address,
            data,
            Default::default(),
            "upload sample contract for checking things out",
        )
        .await?;

    let code_id = upload_res.code_id;
    println!("code id is: {}", code_id);

    println!("initialising the smart contract...");
    let init_result = client
        .instantiate(
            address,
            code_id,
            &InstantiateMsg {},
            "dummy contract".to_string(),
            Default::default(),
            "upload sample contract for checking things out",
            None,
        )
        .await?;

    let contract_address = init_result.contract_address;
    println!("the contract address is: {}", contract_address);

    Ok(contract_address)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. startup a local (or not) chain
    // 2. create an account with non-zero amount of tokens so that it could pay for the minimal tx fees
    // 3. specify the below mnemonic, bech32 prefix, denom and rpc address of the validator
    // 4. compile the contract with `RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown` from inside 'dummy-contract' directory
    // 5. attempt to run this code

    let mnemonic = "UNSPECIFIED"; // bip39 mnemonic
    let bech32_prefix = "UNSPECIFIED";
    let denom = "UNSPECIFIED";
    let validator_rpc = "http://localhost:26657";
    let gas_price = GasPrice::new_with_default_price(denom)?;

    let wallet = DirectSecp256k1HdWallet::from_mnemonic(
        bech32_prefix,
        mnemonic.parse().expect("failed to parse provided mnemonic"),
    )?;

    let client = connect_with_signer(validator_rpc, wallet, gas_price)?;
    let accounts = client.signer().try_derive_accounts()?;
    let address = accounts[0].address();

    let contract = upload_and_initialise(&client, address).await?;

    println!("executing the dummy transaction...");
    let execution_res = client
        .execute(
            address,
            &contract,
            &ExecuteMsg::EmitData {},
            Default::default(),
            "executing emit data",
            vec![],
        )
        .await?;

    let tx_hash = execution_res.transaction_hash;
    println!("the transaction hash is: {}", tx_hash);

    let block = client.get_tx(tx_hash).await?;
    let data = block.tx_result.data;
    println!("the tx_result.data is: {:?}", data);

    let deserialization_attempt =
        proto::cosmwasm::wasm::v1::MsgExecuteContractResponse::decode(data.value().as_ref())?;

    println!("deserialized it to {:?}", deserialization_attempt);

    Ok(())
}
