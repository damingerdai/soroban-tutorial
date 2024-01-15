use soroban_sdk::{Env, Address, testutils::{ Address as _ }};

use crate::{IncrementContractClient, IncrementContract};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, IncrementContract);
    let clinet = IncrementContractClient::new(&env, &contract_id);

    let user_1 = Address::generate(&env);
    let user_2 = Address::generate(&env);

    assert_eq!(clinet.increment(&user_1, &5), 5);
   
}
