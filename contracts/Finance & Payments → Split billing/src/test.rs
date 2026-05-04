#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, Vec};
    use crate::{StreetSplitContract, StreetSplitContractClient};

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract_id = env.register(StreetSplitContract, ());
        let client = StreetSplitContractClient::new(&env, &contract_id);

        let a = Address::random(&env);
        let b = Address::random(&env);

        let participants = Vec::from_array(&env, [a.clone(), b.clone()]);

        client.create_split(&1, &a, &100, &participants);
        client.pay_share(&1, &a);
        client.pay_share(&1, &b);

        assert!(client.is_complete(&1));
    }

    #[test]
    #[should_panic]
    fn test_double_payment() {
        let env = Env::default();
        let contract_id = env.register(StreetSplitContract, ());
        let client = StreetSplitContractClient::new(&env, &contract_id);

        let a = Address::random(&env);
        let participants = Vec::from_array(&env, [a.clone()]);

        client.create_split(&1, &a, &100, &participants);
        client.pay_share(&1, &a);
        client.pay_share(&1, &a); // should panic
    }

    #[test]
    fn test_state_update() {
        let env = Env::default();
        let contract_id = env.register(StreetSplitContract, ());
        let client = StreetSplitContractClient::new(&env, &contract_id);

        let a = Address::random(&env);
        let participants = Vec::from_array(&env, [a.clone()]);

        client.create_split(&1, &a, &100, &participants);
        client.pay_share(&1, &a);

        assert!(client.is_complete(&1));
    }

    #[test]
    fn test_not_complete() {
        let env = Env::default();
        let contract_id = env.register(StreetSplitContract, ());
        let client = StreetSplitContractClient::new(&env, &contract_id);

        let a = Address::random(&env);
        let b = Address::random(&env);
        let participants = Vec::from_array(&env, [a.clone(), b.clone()]);

        client.create_split(&1, &a, &100, &participants);
        client.pay_share(&1, &a);

        assert!(!client.is_complete(&1));
    }

    #[test]
    fn test_multiple_splits() {
        let env = Env::default();
        let contract_id = env.register(StreetSplitContract, ());
        let client = StreetSplitContractClient::new(&env, &contract_id);

        let a = Address::random(&env);
        let participants = Vec::from_array(&env, [a.clone()]);

        client.create_split(&1, &a, &100, &participants);
        client.create_split(&2, &a, &200, &participants);

        client.pay_share(&1, &a);

        assert!(client.is_complete(&1));
        assert!(!client.is_complete(&2));
    }
}