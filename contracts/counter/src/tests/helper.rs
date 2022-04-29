use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Coin, MessageInfo, OwnedDeps, Response, SystemResult,
};

use crate::{
    contract::instantiate,
    msg::{InstantiateMsg, QueryMsg},
    tests::query_mocks::custom_query_execute,
};

pub fn instantiate_default() -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier>,
    MessageInfo,
    Response,
) {
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg { count: 25 };
    let info = mock_info("creator", &vec![]);
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    (deps, info, res)
}

pub fn instantiate_with_custom_querier() -> (
    OwnedDeps<MockStorage, MockApi, MockQuerier<QueryMsg>>,
    MessageInfo,
    Response,
) {
    let mut deps = custom_mock_dependencies(&[]);
    let msg = InstantiateMsg { count: 25 };
    let info = mock_info("creator", &vec![]);
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    (deps, info, res)
}

fn custom_mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<QueryMsg>> {
    let custom_querier: MockQuerier<QueryMsg> =
        MockQuerier::new(&[("counter_contract", contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(query)));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}
