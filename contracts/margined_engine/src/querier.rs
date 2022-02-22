// Contains queries for external contracts
use cosmwasm_std::{to_binary, DepsMut, QueryRequest, StdResult, Uint128, WasmQuery};
use cosmwasm_bignumber::{Decimal256, Uint256};
use margined_perp::margined_vamm::{Direction, QueryMsg, StateResponse};

// returns the state of the request vamm
// can be used to calculate the input and outputs
pub fn _query_vamm_state(deps: &DepsMut, address: String) -> StdResult<StateResponse> {
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: address,
        msg: to_binary(&QueryMsg::State {})?,
    }))
}

// returns the state of the request vamm
// can be used to calculate the input and outputs
pub fn query_vamm_output_price(
    deps: &DepsMut,
    address: String,
    direction: Direction,
    amount: Uint128,
) -> StdResult<Uint128> {
    let amount256:Decimal256 = Decimal256::from_uint256(Uint256::from(amount));
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: address,
        msg: to_binary(&QueryMsg::OutputPrice { direction, amount: amount256 })?,
    }))
}
