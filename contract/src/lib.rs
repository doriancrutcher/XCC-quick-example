// Set Imports Here 
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, log,
    serde::{Deserialize, Serialize},
    Balance, Promise, PromiseResult, PanicOnDefault
};
use near_sdk::{env, near_bindgen, setup_alloc,AccountId, Gas, serde_json};
use near_sdk::collections::{LookupMap,UnorderedSet};

setup_alloc!();


// Set variables for XCC here
const GAS_FOR_ACCOUNT_CALLBACK: Gas = 11_000_000_000_000;

//Set Traits Here 
//Two kinds of traits for your external contracts
// The reference to the contract you are pulling your methods from 
// 1. #[ext_contract(name_of_ext_contract)]
// The reference to "ourselves" at a callback
// 2. #[ext_contract(ext_self)] 

#[ext_contract(ext_whitelist)]
pub trait WhiteListContract{
    // This acts as an interface to the external contract we are calling from this one.
    // We simply put the function signature that we are calling
    fn is_whitelisted(&self,staking_pool_account_id:AccountId)->bool;
}

#[ext_contract(ext_self)]
pub trait ExtSelf{
    // Here we put the contract signature of our local contract
    // picture this as a cross contract call to ourselves
    fn callback_promise_result()->bool;
}


// Define Your Contract Struct 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    records: LookupMap<String, String>,
}

// Initialize implementation for contract  
#[near_bindgen]
impl Contract{
    //declare your initialization function to init your contract
    #[init]
    pub fn new(owner:AccountId)->Self{
        Self{
            records:LookupMap::new(b"c")
        }
    }

    // setup internal functions to handle callback
    pub fn is_whitelisted_callback(&self)->Promise{
        // anatomy of XCC 
        // contract_refernce_from_trait::method_from_contract(arg1,arg2,"contract_name".to_string(),Deposit,Gas)
        ext_whitelist::is_whitelisted("your-account-name.testnet".to_string(),&String::from("whitelist.your-account-name.testnet"),0,GAS_FOR_ACCOUNT_CALLBACK).then(
            ext_self::callback_promise_result(
                &env::current_account_id(),
                0,
                GAS_FOR_ACCOUNT_CALLBACK
            )
        )

    }

    pub fn callback_promise_result(
        &self,   
    )->bool{
        // assert that a promise in fact exists after making a call
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expected 1 promise Result"
        );

       

        match env::promise_result(0){
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(val) => {
                if let Ok(is_whitelisted) = near_sdk::serde_json::from_slice::<bool>(&val) {
                    is_whitelisted
                } else {
                    env::panic(b"ERR_WRONG_VAL_RECEIVED")
                }
            } ,
            PromiseResult::Failed => env::panic(b"Hey Dorian:ERR_CALL_FAILED"),
        
        }
        
    }

}


