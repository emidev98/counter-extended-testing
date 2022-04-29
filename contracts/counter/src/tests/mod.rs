#[cfg(test)]
mod env_mocks;
#[cfg(test)]
mod helper;
#[cfg(test)]
mod query_mocks;

#[cfg(test)]
mod test {
    use crate::contract::{execute, query};
    use crate::msg::{ExecuteMsg, LastResetTime, QueryMsg};
    use crate::tests::env_mocks::custom_mocked_env;
    use crate::tests::helper::{instantiate_default, instantiate_with_custom_querier};
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{
        from_binary, Addr, MessageInfo, QuerierWrapper, QueryRequest, Response, Timestamp,
    };
    use std::vec;

    #[test]
    fn proper_initialization() {
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

    #[test]
    fn reset_with_custom_mocked_env() {
        // GIVEN
        let (mut deps, info, _) = instantiate_default();

        // WHEN
        let msg = ExecuteMsg::Reset { count: 5 };
        execute(deps.as_mut(), custom_mocked_env(), info, msg).unwrap();

        // THEN
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetLastResetTime {}).unwrap();
        let value: LastResetTime = from_binary(&res).unwrap();
        assert_eq!(Some(Timestamp::from_nanos(10)), value.last_reset_time);
    }

    #[test]
    fn query_mocked_last_reset_time() {
        // GIVEN
        let (deps, _, _) = instantiate_with_custom_querier();
        let req: QueryRequest<_> = QueryMsg::GetLastResetTime {}.into();

        // WHEN
        let wrapper = QuerierWrapper::new(&deps.querier);
        let response: LastResetTime = wrapper.custom_query(&req).unwrap();

        // THEN
        assert_eq!(Some(Timestamp::from_nanos(420)), response.last_reset_time);
    }
}