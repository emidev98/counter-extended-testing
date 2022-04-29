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

    /**
        Validate that instantiate_default() return the expected values, this method
        is useful because it ensures that the data inside instantiate_default() isn't modified
        which risks breaking further tests. If this test fails it indicates that something have
        been modified inside the helper.
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

    /**
        This test confirms that block time is stored properly as a result of custom_mocked_env()
        when the method ExecuteMsg:Reset is executed.
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
}
