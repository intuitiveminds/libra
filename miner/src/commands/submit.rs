// `submit` subcommand

use abscissa_core::{Command, Options, Runnable};
use crate::{block::Block, prelude::*};
use libra_types::{waypoint::Waypoint, account_address::AccountAddress, transaction::authenticator::AuthenticationKey};
use libra_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519PublicKey, Ed25519Signature},
    test_utils::KeyPair,
    PrivateKey,
};
// use libra_crypto::test_utils::KeyPair;
use anyhow::Error;
// use client::{
//     account::{Account, AccountData, AccountTypeSpecifier},
//     keygen::KeyGen,
// };
use cli::{libra_client::LibraClient, AccountData, AccountStatus};
use reqwest::Url;
use std::{thread, path::PathBuf, time, fs, io::BufReader};
use libra_config::config::NodeConfig;
use libra_types::transaction::{Script, TransactionArgument, TransactionPayload};
use libra_types::{vm_error::StatusCode, transaction::helpers::*};
use crate::delay::delay_difficulty;
use stdlib::transaction_scripts;

#[derive(Command, Debug, Default, Options)]
pub struct SubmitCmd {
    #[options(help = "Provide a waypoint for the libra chain")]
    waypoint: String, //Option<Waypoint>,

    #[options(help = "Path of swarm config directory.")]
    path: PathBuf,

    #[options(help = "Already mined height to submit")]
    height: usize,
}

impl Runnable for SubmitCmd {
    fn run(&self) {
        println!("TESTING SUBMITTING WITH KEYPAIR TO SWARM");

        // submit_noop(self.path.clone(), self.height.clone());

        match submit_test(self.path.clone(), self.height.clone()){
            Ok(res) => {
                println!("Ok: {}", &res)
            }
            Err(err) => {
                println!("Err: {}", &err)

            }
        };
    }

}

fn submit_test(mut config_path: PathBuf, height_to_submit: usize ) -> Result<String, Error> {
    let miner_configs = app_config();
    let mut tower_height: usize = 1;

    // let file = fs::File::open(format!("{:?}/block_{}.json", &miner_configs.get_block_dir(), height_to_submit)).expect("Could not open block file");

    // let file = fs::File::open("./blocks/block_1.json").expect("Could not open block file");
    // let reader = BufReader::new(file);
    // let block: Block = serde_json::from_reader(reader).unwrap();
    // let challenge = block.preimage;
    // let proof = block.data;

    // NOTE: these fixtures are exactly what is submitted in the e2e test.
    //for comparison, we have the e2e test for the exact same script here: language/e2e-tests/src/tests/ol_e2e_test_redeem.rs
    // which you can run with cargo xtest -p language-e2e-tests ol_e2e_test_redeem -- --nocapture
    let challenge = hex::decode("a0f9e7a5483616060676b3937b1afdd4eae29adb9762391b066c4e3b48770f1f").unwrap();

    let proof = hex::decode("0042f825f7a9471eef2adbfac1e43141106739d18a613d7493f6844d1f72b4bf9fb3b6c2a028e51468776d2c8d9c81ae96e3ef9a895b0be36521d5d80a9a28f4194d32b1692e99afb9c2c3c32a3937a79c8a9ec3bb87b104fd19bc1fc56574551eec24c0c1d94d4bdb63db208712338f987e5fb904da0af1f698a361ad06781885be7dc71048fc137cc7ec6276306ff7a9f0e37cee05c4e74a8e9a0fc61ffb09737dbe613a1cdfd83bd48426be9db75067b1008226e2cb393da0129355095aeacceaa5fb67974f82294d88399c234b414e41586ac71229a214057ca147e1dd80fb2fac38d15fce07fced0896e116ab6226ac70f3109ac3a88c3df38f6279a565f2ffd90ee20d2ff04778ea7a818bc1b6134d121a7701724fe525e2f3dcfbf9d1f0dcd97034f8c7a71a3b91e95f3a63d8c97bdbf3f970156dffb767278af20af51efeec80117474b29a704a38aa5a76412000cbb31c9143446b5319bae841084c7ad03c3aa1455d42ed57842b0f9b1abb81a262d855969340916df435fb7108629d1e9a3d49a9c5a84003d05087b5961a5552be15e81d58ee80f068a00636585a139dd58ddcde568f5253ceb31a1b1248e4897e2ae459fd8d232d0c8ff688c90f509cc413e3ffc55a3764ab2a80ce67c012296a1337141df0a47daa86f957b769cff6ad26d8685c43fa0a14c678d7e1875ddd15df3c0bf463c81fc330c71f173dbc2900388f9d8df24781b0b7404318a938a6f7a2dc2e8e92d1224ac410fe36af338e27f57d2efb06cfba995ec5a0c23cdf8d2e889988294b58f9507faa5c758f25f612f85e70f6754ff0f4539b2198bcde603b5a5fe0adc51719f8584214657ef78b95e7cd3a1dbf9ea01102402171588d6c80f617b2cb521e533c4abacac46766df58381bce43a56ee8682297b44367a046d5a934c2838d7bead395e9418df1c63b47eb958bdaab50b898ed2ac717d0b2476f1621661d9557e1b70f15cfde15cad4a15268ff7e57edb0b1e2339f264c5226b411cdd9bdcb002f59922fdf9aa1cd38056b0fe69fbdc639201f234f99949118c4bedeb8ea912f382bd0382c032c7551a1ffeba2f9f30a41c062d563e5bbdf961c7c9cc49ee618601003457b09e5e0de3ae01a23f3c8e8356a3dd73d9c0214c31a79c011d832f94176c45aff9035090f7246e157887180ca4be90fcf3e1f6d7a902f20b73401187949a78b3e58d6e7fa01caaea98836237a7a53c52308c8b91728c9ada6acfe83d796b427511c37f11d66a65c15779e35a7db06faed5bb0c8d2c5444107074741d09e46aa004c02376b619fb84ef9c591ffd4209027dbfea4dd0070a8f7c2f4b5baa09da23e118e0e680b7fb571d6642fe4604656d6ebd0dfa6e7d89187ba87f929b8b606c8c1eec7f0f11049ad648684a93d84c74336678bf0acb8884112301e66e9b06828b8dda061871f").unwrap();

    config_path.push("../saved_logs/0/node.config.toml");

    let config = NodeConfig::load(&config_path)
        .unwrap_or_else(|_| panic!("Failed to load NodeConfig from file: {:?}", config_path));
    match &config.test {
        Some( conf) => {
            println!("Swarm Keys : {:?}", conf);
            tower_height = 0;
        },
        None =>{
            println!("test config does not set.");
        }
    }
    
    // TODO (LG): When we are not testing swarm.
    // let mut is_prod = true;
    // if is_prod {
    //     let hex_literal = format!("0x{}", &miner_configs.profile.account);
    //     let account_address = AccountAddress::from_hex_literal(&hex_literal).unwrap();
    //     dbg!(&account_address);
        
    //     let url = miner_configs.chain_info.node.as_ref().unwrap().parse::<Url>();
    //     // let url: Result<Url, Error> = miner_configs.chain_info.node;
    //     let parsed_waypoint: Result<Waypoint, Error> = miner_configs.chain_info.base_waypoint.parse();
        
    //     //unwrap().parse::<Waypoint>();
    //     let auth_key = &miner_configs.profile.auth_key;
    //     dbg!(auth_key);
    //     let privkey = &miner_configs.profile.operator_private_key;
    //     tower_height = height_to_submit;
    //     // let operator_keypair = Some(AccountKeyPair::load(privkey));
    //     dbg!(privkey);
    // }
    

    // Create a client object
    let mut client = LibraClient::new(
        Url::parse(format!("http://localhost:{}", config.rpc.address.port()).as_str()).unwrap(),
        config.base.waypoint.waypoint_from_config().unwrap().clone()
    ).unwrap();

    
    let mut private_key = config.test.unwrap().operator_keypair.unwrap();
    let auth_key = AuthenticationKey::ed25519(&private_key.public_key());

    let address = auth_key.derived_address();
    let account_state = client.get_account_state(address.clone(), true).unwrap();
    dbg!(&account_state);


    let mut sequence_number = 0u64;
    if account_state.0.is_some() {
        sequence_number = account_state.0.unwrap().sequence_number;
    }
    dbg!(&sequence_number);

    // Create the unsigned MinerState transaction script
    let script = Script::new(
        transaction_scripts::StdlibScript::Redeem.compiled_bytes().into_vec(),
        vec![],
        vec![
            TransactionArgument::U8Vector(challenge),
            TransactionArgument::U64(delay_difficulty()),
            TransactionArgument::U8Vector(proof),
            TransactionArgument::U64(tower_height as u64),
        ],
    );

    // Doing a no-op transaction here which will print
    // [debug] 000000000000000011e110  in the logs if successful.
    // NoOp => "ol_no_op.move",

    // let script = Script::new(
    //     transaction_scripts::StdlibScript::NoOp.compiled_bytes().into_vec(),
    //     vec![],
    //     vec![
    //         // TransactionArgument::U8Vector(challenge),
    //         // TransactionArgument::U64(delay_difficulty()),
    //         // TransactionArgument::U8Vector(proof),
    //         // TransactionArgument::U64(tower_height as u64),
    //     ],
    // );




    let keypair = KeyPair::from(private_key.take_private().clone().unwrap());
    dbg!(&keypair);
    // Plz Halp (ZM):
    // sign the transaction script
    let txn = create_user_txn(
        &keypair,
        TransactionPayload::Script(script),
        address,
        sequence_number,
        700_000,
        0,
        "GAS".parse()?,
        5000000, // for compatibility with UTC's timestamp.
    )?;

    // Plz Halp  (ZM):
    // get account_data struct
    let mut sender_account_data = AccountData {
        address,
        authentication_key: Some(auth_key.to_vec()),
        key_pair: Some(keypair),
        sequence_number,
        status: AccountStatus::Persisted,
    };

    dbg!(&sender_account_data);
    // Plz Halp (ZM):
    // // Submit the transaction with libra_client
    match client.submit_transaction(
        Some(&mut sender_account_data),
        txn
    ){
        Ok(_) => {
            ol_wait_for_tx(address, sequence_number, &mut client);
            Ok("Tx submitted".to_string())

        }
        Err(err) => Err(err)
    }

    // TODO (LG): Make synchronous to libra client.

    // Ok(())
    // Ok("Succcess".to_owned())
}

fn submit_noop(mut config_path: PathBuf, height_to_submit: usize ) -> Result<String, Error> {

    config_path.push("../saved_logs/0/node.config.toml");

    let config = NodeConfig::load(&config_path)
        .unwrap_or_else(|_| panic!("Failed to load NodeConfig from file: {:?}", config_path));
    match &config.test {
        Some( conf) => {
            println!("Swarm Keys : {:?}", conf);
        },
        None =>{
            println!("test config does not set.");
        }
    }

    // Create a client object
    let mut client = LibraClient::new(
        Url::parse(format!("http://localhost:{}", config.rpc.address.port()).as_str()).unwrap(),
        config.base.waypoint.waypoint_from_config().unwrap().clone()
    ).unwrap();

    
    let mut private_key = config.test.unwrap().operator_keypair.unwrap();
    let auth_key = AuthenticationKey::ed25519(&private_key.public_key());

    let address = auth_key.derived_address();
    let account_state = client.get_account_state(address.clone(), true).unwrap();
    dbg!(&account_state);


    let mut sequence_number = 0u64;
    if account_state.0.is_some() {
        sequence_number = account_state.0.unwrap().sequence_number;
    }
    dbg!(&sequence_number);

    // Doing a no-op transaction here which will print
    // [debug] 000000000000000011e110  in the logs if successful.
    // NoOp => "ol_no_op.move",

    let script = Script::new(
        transaction_scripts::StdlibScript::NoOp.compiled_bytes().into_vec(),
        vec![],
        vec![
            // TransactionArgument::U8Vector(challenge),
            // TransactionArgument::U64(delay_difficulty()),
            // TransactionArgument::U8Vector(proof),
            // TransactionArgument::U64(tower_height as u64),
        ],
    );

    let keypair = KeyPair::from(private_key.take_private().clone().unwrap());
    dbg!(&keypair);
    // Plz Halp (ZM):
    // sign the transaction script
    let txn = create_user_txn(
        &keypair,
        TransactionPayload::Script(script),
        address,
        sequence_number,
        700_000,
        0,
        "GAS".parse()?,
        5_000_000, // for compatibility with UTC's timestamp.
    )?;

    // Plz Halp  (ZM):
    // get account_data struct
    let mut sender_account_data = AccountData {
        address,
        authentication_key: Some(auth_key.to_vec()),
        key_pair: Some(keypair),
        sequence_number,
        status: AccountStatus::Persisted,
    };

    dbg!(&sender_account_data);
    // Plz Halp (ZM):
    // // Submit the transaction with libra_client
    match client.submit_transaction(
        Some(&mut sender_account_data),
        txn
    ){
        Ok(_) => {
            ol_wait_for_tx(address, sequence_number, &mut client);
            Ok("Tx submitted".to_string())

        }
        Err(err) => Err(err)
    }

    // TODO (LG): Make synchronous to libra client.

    // Ok(())
    // Ok("Succcess".to_owned())
}

fn ol_wait_for_tx (
    sender_address: AccountAddress,
    sequence_number: u64,
    client: &mut LibraClient) -> Result<(), Error>{
        // if sequence_number == 0 {
        //     println!("First transaction, cannot query.");
        //     return Ok(());
        // }

        let mut max_iterations = 10;
        println!(
            "waiting for tx from acc: {} with sequence number: {}",
            sender_address, sequence_number
        );

        loop {
            println!("test");
        //     stdout().flush().unwrap();

        //     // TODO: first transaction in sequence fails


            match &mut client
                .get_txn_by_acc_seq(sender_address, sequence_number - 1, true)
            {
                Ok(Some(txn_view)) => {
                    print!("txn_view: {:?}", txn_view);
                    if txn_view.vm_status == StatusCode::EXECUTED {
                        println!("transaction executed!");
                        if txn_view.events.is_empty() {
                            println!("no events emitted");
                        }
                        break Ok(());
                    } else {
                        // break Err(format_err!(
                        //     "transaction failed to execute; status: {:?}!",
                        //     txn_view.vm_status
                        // ));

                        break Ok(());

                    }
                }
                Err(e) => {
                    println!("Response with error: {:?}", e);
                }
                _ => {
                    print!(".");
                }
            }
            max_iterations -= 1;
        //     if max_iterations == 0 {
        //         panic!("wait_for_transaction timeout");
        //     }
            thread::sleep(time::Duration::from_millis(100));
    }
}