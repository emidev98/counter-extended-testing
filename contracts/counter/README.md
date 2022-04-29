Table of content 
- Context
- 1.Testing
    - 1.1.Folder structure
    - 1.2.Testing structure
- 2.Helper functions
- 3.Mocks
- 4.Testing functions

# Context

Smart contract based on [cw-template "Counter"](https://github.com/InterWasm/cw-template) with a slightly modified property in **State.last_reset** which contains the **timestamp** of the block when **reset** have been executed. 

This modification will allow to cover more test cases as it is the propose of this repo.

# 1.Testing

This guide will give you an idea on how to 
- structure your tests, 
- create a unit test,
- mock data,
- and how to use helpers.

## 1.1.Folders structure

The tests have been moved from contract.rs to the tests module with many sub modules: 

```
.
├── contract.rs         // Entry points of the smart contract     
└── tests               // Module that contains ONLY test-related code
    ├── env_mocks.rs    // Example mock for the environment 
    ├── helper.rs       // Common functions to avoid unnecessary code duplication
    ├── mod.rs          // Module entry point where tests are located
    └── query_mocks.rs  // Mocked QueryMsg data
```


## 1.2.Testing structure

The approach that follows this guide is [Given-When-Then](https://en.wikipedia.org/wiki/Given-When-Then) because it keeps testing code simple, robust and is easy to detect when to create helpers.

- GIVEN: pre-requisites,
- WHEN: usage of the pre-requisites to complete an action,
- THEN: assert the previous statement have been executed correctly.

# 2.Helper functions

Common functions that follows the [KISS principle](https://en.wikipedia.org/wiki/KISS_principle) to avoid complexity and code duplication.

```Rust
/** 
    Instantiate the contract with count as 25 and owner as "creator".
    
    Returns contract dependencies, the message that was send to instantiate 
    the contract and the response from instantiate method. 

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
```


```Rust
/** 
    Instantiate the contract with count as 25 and owner as "creator".
    
    Returns contract dependencies with MockQuerier used to mock the 
    requests to QueryMsg, the message that was send to instantiate 
    the contract and the response from instantiate method. 

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
    Private function used to inject the custom_query_msg with mocked data.
    Mocked data can be found in a different file because depending of your
    smart contract can ba very large file.

    Returns contract dependencies with custom_mocked_data
**/
fn custom_mock_dependencies(contract_balance: &[Coin]) -> 
    OwnedDeps<MockStorage, MockApi, MockQuerier<QueryMsg>> 
{
    let custom_querier: MockQuerier<QueryMsg> =
        MockQuerier::new(&[("counter_contract", contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(crate::tests::query_mocks::custom_query_msg(query)));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

```

# 3.Mocks

As you can observe on this guide there are two files defined for mocks containing data to be asserted:

- env_mocks: contains contract and block mocked data to assert functionalities that depends on block height, time or address.

```Rust
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
```

- query_mocks: contains mocked data abut incoming query requests which can be useful when two smart contracts are chained and you do not want to instantiate two smart contract to test only one or when you want to validate a case that is created by multiple steps.

```Rust
use crate::msg::{CountResponse, LastResetTime, QueryMsg};
use cosmwasm_std::{to_binary, ContractResult, QueryResponse, Timestamp};

pub fn custom_query_msg(query: &QueryMsg) -> ContractResult<QueryResponse> {
    let msg = match query {
        QueryMsg::GetCount {} => to_binary(&CountResponse { count: 69 }),
        QueryMsg::GetLastResetTime {} => to_binary(&LastResetTime {
            last_reset_time: Some(Timestamp::from_nanos(420)),
        }),
    };
    msg.into()
}
```

# 4.Testing functions

As previously mentioned the code used to test the Counter contract will be located inside mod.rs. As this is an example with very few tests there is no necessity to create more files, but if your test cases grow would be recommendable to create multiple modules based on your tests context.

```Rust

/**
    Validate that instantiate_default() return the expected values,
    this method is useful because as it us a helper you may be tempted
    to modify the data inside instantiate_default() which can break
    further tests. This way if you see this test failing you can 
    find that it is because something have been modified inside the helper. 
**/
#[test]
fn proper_initialization() {
    // GIVEN properties inside instantiate_default()

    // WHEN
    let (_s, info, res) = instantiate_default();

    // THEN
    assert_eq!(
        Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("owner", "creator")
            .add_attribute("count", "25"),
        res
    );
    assert_eq!(
        MessageInfo {
            sender: Addr::unchecked("creator"),
            funds: vec![]
        },
        info
    );
}

```


```Rust
    /**
        This test confirms that block time is stored correctly thanks to custom_mocked_env(),
        when the method ExecuteMsg:Reset is executed. As you can observe there is no other
        query to validate that count was set successfully to 5 because based on KISS principle
        you should build another test to validate that use-case.
    **/
    #[test]
    fn reset_with_custom_mocked_env() {
        // GIVEN
        let (mut deps, info, _) = instantiate_default();
        let msg = ExecuteMsg::Reset { count: 5 };

        // WHEN
        execute(deps.as_mut(), custom_mocked_env(), info, msg).unwrap();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetLastResetTime {}).unwrap();
        let value: LastResetTime = from_binary(&res).unwrap();

        // THEN
        assert_eq!(Some(Timestamp::from_nanos(10)), value.last_reset_time);
    }
```


```Rust
    /**
        This test allows you to understand how a CustomQuerier can be configured in order to
        bypass previous steps as for example ExecuteMsg::Reset used in the previous test. 

        To explain the content of this test you can see that there is no call to Reset but the
        GetLastResetTime query doest not return None because a mock have been setup which will always
        return the values from the method query_mocks::custom_query_msg(...)
    */
    #[test]
    fn query_mocked_last_reset_time() {
        // GIVEN
        let (deps, _, _) = instantiate_with_custom_querier();
        let req: QueryRequest<_> = QueryMsg::GetLastResetTime {}.into();
        let wrapper = QuerierWrapper::new(&deps.querier);

        // WHEN
        let response: LastResetTime = wrapper.custom_query(&req).unwrap();

        // THEN
        assert_eq!(Some(Timestamp::from_nanos(420)), response.last_reset_time);
    }
```