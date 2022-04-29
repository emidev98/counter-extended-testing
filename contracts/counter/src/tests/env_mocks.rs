use cosmwasm_std::{Env, BlockInfo, Timestamp, ContractInfo, Addr};


pub fn custom_mocked_env() -> Env {
    Env {
        block: BlockInfo {
            height: 420,
            time: Timestamp::from_nanos(10),
            chain_id: "never_went_down".to_string(),
        },
        contract: ContractInfo {
            address: Addr::unchecked("counter_contract"),
        },
    }
}