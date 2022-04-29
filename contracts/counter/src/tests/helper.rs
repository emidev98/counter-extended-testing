use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Coin, MessageInfo, OwnedDeps, Response, SystemResult,
};

use crate::{
    contract::instantiate,
    msg::{InstantiateMsg, QueryMsg},
};

/**
    Instantiate the contract with count = 25 and owner = "creator".

    Returns the contract dependencies, instantiation message and response from
    instantiate method.
**/

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

/**
    Instantiate the contract with count = 25 and owner = "creator".

    Returns contract dependencies (including the QueryMsg mocked by MockQuerier),
    the instantiation message and the response from instantiate method.
**/

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

/**
    Private function used to inject the custom_query_msg into the mocked data.
    This mocked data is stored in a different file because filze size contraints.

    Returns contract dependencies with custom_mocked_data
**/
fn custom_mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<QueryMsg>> {
    let custom_querier: MockQuerier<QueryMsg> =
        MockQuerier::new(&[("counter_contract", contract_balance)]).with_custom_handler(|query| {
            SystemResult::Ok(crate::tests::query_mocks::custom_query_msg(query))
        });

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}
