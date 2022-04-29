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
