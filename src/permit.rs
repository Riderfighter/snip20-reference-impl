#![allow(clippy::field_reassign_with_default)] // This is triggered in `#[derive(JsonSchema)]`

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr, Uint128};

#[derive(Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Permit {
    pub params: PermitParams,
    pub signature: PermitSignature,
}

#[derive(Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PermitParams {
    pub allowed_tokens: Vec<HumanAddr>,
    pub permit_name: String,
    pub chain_id: String,
    pub permissions: Vec<Permission>,
}

#[derive(Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PermitSignature {
    pub pub_key: PubKey,
    pub signature: Binary,
}

#[derive(Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PubKey {
    /// ignored, but must be "tendermint/PubKeySecp256k1" otherwise the verification will fail
    pub r#type: String,
    /// Secp256k1 PubKey
    pub value: Binary,
}

// Note: The order of fields in this struct is important for the permit signature verification!
#[remain::sorted]
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SignedPermit {
    /// ignored
    pub account_number: Uint128,
    /// ignored, no Env in query
    pub chain_id: String,
    /// ignored
    pub fee: Fee,
    /// ignored
    pub memo: String,
    /// the signed message
    pub msgs: Vec<PermitMsg>,
    /// ignored
    pub sequence: Uint128,
}

impl SignedPermit {
    pub fn from_params(params: &PermitParams) -> Self {
        Self {
            account_number: Uint128::zero(),
            chain_id: params.chain_id.clone(),
            fee: Fee::new(),
            memo: String::new(),
            msgs: vec![PermitMsg::from_content(PermitContent::from_params(params))],
            sequence: Uint128::zero(),
        }
    }
}

// Note: The order of fields in this struct is important for the permit signature verification!
#[remain::sorted]
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Fee {
    pub amount: Vec<Coin>,
    pub gas: Uint128,
}

impl Fee {
    pub fn new() -> Self {
        Self {
            amount: vec![Coin::new()],
            gas: Uint128(1),
        }
    }
}

// Note: The order of fields in this struct is important for the permit signature verification!
#[remain::sorted]
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Coin {
    pub amount: Uint128,
    pub denom: String,
}

impl Coin {
    pub fn new() -> Self {
        Self {
            amount: Uint128::zero(),
            denom: "uscrt".to_string(),
        }
    }
}

// Note: The order of fields in this struct is important for the permit signature verification!
#[remain::sorted]
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PermitMsg {
    pub r#type: String,
    pub value: PermitContent,
}

impl PermitMsg {
    pub fn from_content(content: PermitContent) -> Self {
        Self {
            r#type: "query_permit".to_string(),
            value: content,
        }
    }
}

// Note: The order of fields in this struct is important for the permit signature verification!
#[remain::sorted]
#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PermitContent {
    pub allowed_tokens: Vec<HumanAddr>,
    pub permissions: Vec<Permission>,
    pub permit_name: String,
}

impl PermitContent {
    pub fn from_params(params: &PermitParams) -> Self {
        Self {
            allowed_tokens: params.allowed_tokens.clone(),
            permit_name: params.permit_name.clone(),
            permissions: params.permissions.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    Allowance,
    Balance,
    History,
}
