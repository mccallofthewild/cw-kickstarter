use cosmwasm_std::{Coin, CosmosMsg, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub denom: String,
    pub goal: Uint128,
    pub start: Option<Timestamp>,
    pub deadline: Timestamp,
    pub name: String,
    pub description: String,
    pub execute_msg: Option<CosmosMsg>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // fund the project with a given amount of tokens
    // receives coins from `WasmExecuteMsg.funds`
    Fund {},
    // execute the project if the goal is reached
    Execute {},
    // refund the project if the goal is not reached
    Refund {},
    // claim the project's funds if the goal is reached
    Claim {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    // * `get_shares`: returns a user's shares in the project.
    GetShares {
        user: String,
    },
    // returns a list of all funders and their shares.
    GetFunders {
        limit: Uint128,
        start_after: Option<String>,
    },
    // returns total fund held by contract.
    GetTotalFunds {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryResponse {
    // returns config
    GetConfigResponse {
        goal: Coin,
        deadline: Timestamp,
        name: String,
        description: String,
    },
    // returns a user's shares in the project.
    GetSharesResponse {
        address: String,
        shares: Uint128,
    },
    // returns a list of all funders and their shares.
    GetFundersResponse {
        funders: Vec<(String, Uint128)>,
    },
    // Get Total Funds Response
    GetTotalFundsResponse {
        total_funds: Coin,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}
