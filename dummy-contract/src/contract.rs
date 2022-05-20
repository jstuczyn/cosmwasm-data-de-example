use common::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response};

type ContractError = String;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut<'_>,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let data = 1u64;
    Ok(Response::new().set_data(data.to_be_bytes()))
}

#[entry_point]
pub fn query(_deps: Deps<'_>, _env: Env, _msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    Ok(QueryResponse::default())
}

#[entry_point]
pub fn migrate(_deps: DepsMut<'_>, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}
