use candid::{CandidType, Deserialize, Principal};
use core::future::Future;
use ic_cdk::api::call::CallResult;
use ic_cdk_timers::TimerId;
use icrc_ledger_types::icrc1::transfer::TransferArg;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap};
use ic_ledger_types::{TransferArgs, BlockIndex};

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct QueryBlocksRequest {
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Icrc1TransferRequest {
    pub transfer_args: TransferArg,
    pub sweeped_index: Option<u64>,
}

impl Icrc1TransferRequest {
    pub fn new(transfer_args: TransferArg, sweeped_index: Option<u64>) -> Self {
        Self {
            transfer_args,
            sweeped_index,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ToRecord {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Icrc1TransferResponse {
    Ok(u64),
    Err(Error),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Error {
    GenericError(GenericErrorRecord),
    TemporarilyUnavailable,
    BadBurn(BadBurnRecord),
    Duplicate(DuplicateRecord),
    BadFee(BadFeeRecord),
    CreatedInFuture(CreatedInFutureRecord),
    TooOld,
    InsufficientFunds(InsufficientFundsRecord),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GenericErrorRecord {
    message: String,
    error_code: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BadBurnRecord {
    min_burn_amount: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct DuplicateRecord {
    duplicate_of: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BadFeeRecord {
    expected_fee: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreatedInFutureRecord {
    ledger_time: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct InsufficientFundsRecord {
    balance: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QueryBlocksResponse {
    pub certificate: Option<Vec<u8>>,
    pub blocks: Vec<Block>,
    pub chain_length: u64,
    pub first_block_index: u64,
    pub archived_blocks: Vec<ArchivedBlock>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Block {
    pub transaction: Transaction,
    pub timestamp: Timestamp,
    pub parent_hash: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub memo: u64,
    pub icrc1_memo: Option<Vec<u8>>,
    pub operation: Option<Operation>,
    pub created_at_time: Timestamp,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Timestamp {
    pub timestamp_nanos: u64,
}
impl Timestamp {
    pub fn from_nanos(timestamp_nanos: u64) -> Self {
        Self { timestamp_nanos }
    }
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Operation {
    Approve(Approve),
    Burn(Burn),
    Mint(Mint),
    Transfer(Transfer),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Approve {
    pub fee: E8s,
    pub from: Vec<u8>,
    pub allowance_e8s: i64,
    pub allowance: E8s,
    pub expected_allowance: Option<E8s>,
    pub expires_at: Option<Timestamp>,
    pub spender: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Burn {
    pub from: Vec<u8>,
    pub amount: E8s,
    pub spender: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Mint {
    pub to: Vec<u8>,
    pub amount: E8s,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Transfer {
    pub to: Vec<u8>,
    pub fee: E8s,
    pub from: Vec<u8>,
    pub amount: E8s,
    pub spender: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct E8s {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ArchivedBlock {
    pub callback: HashMap<String, Callback>,
    pub start: u64,
    pub length: u64,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Callback {
    Ok { blocks: Vec<Block> },
    Err(CallbackError),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum CallbackError {
    BadFirstBlockIndex {
        requested_index: u64,
        first_valid_index: u64,
    },
    Other {
        error_message: String,
        error_code: u64,
    },
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum SweepStatus {
    Swept,
    FailedToSweep,
    NotSwept,
}

#[derive(Debug, CandidType, Deserialize, Serialize, Clone)]
pub struct StoredTransactions {
    pub index: u64,
    pub memo: u64,
    pub icrc1_memo: Option<Vec<u8>>,
    pub operation: Option<Operation>,
    pub created_at_time: Timestamp,
    pub sweep_status: SweepStatus,
}

// #[derive(CandidType, Deserialize, Serialize, Clone)]
// pub struct PrunedTransactions {
//     pub index: u64,
//     pub memo: u64,
//     pub operation: Option<Operation>,
// }

impl StoredTransactions {
    pub fn new(index: u64, transaction: Transaction) -> Self {
        Self {
            index,
            memo: transaction.memo,
            icrc1_memo: transaction.icrc1_memo,
            operation: transaction.operation,
            created_at_time: transaction.created_at_time,
            sweep_status: SweepStatus::NotSwept,
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Default)]
pub struct StoredPrincipal {
    principal: Option<Principal>,
}

impl StoredPrincipal {
    pub fn new(principal: Principal) -> Self {
        Self {
            principal: Some(principal),
        }
    }

    pub fn get_principal(&self) -> Option<Principal> {
        self.principal
    }
}

use ic_stable_structures::{
    memory_manager::VirtualMemory,
    storable::{Bound, Storable},
    DefaultMemoryImpl,
};

const MAX_VALUE_SIZE: u32 = 500;
impl Storable for StoredTransactions {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap()) // Assuming using Candid for serialization
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).unwrap() // Assuming using Candid for deserialization
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

impl Storable for StoredPrincipal {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap()) // Assuming using Candid for serialization
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        candid::decode_one(bytes.as_ref()).unwrap() // Assuming using Candid for deserialization
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

pub trait TimerManagerTrait {
    fn set_timer(interval: std::time::Duration) -> TimerId;
    fn clear_timer(timer_id: TimerId);
}

pub struct TimerManager;

pub trait InterCanisterCallManagerTrait {
    async fn query_blocks(
        ledger_principal: Principal,
        req: QueryBlocksRequest,
    ) -> CallResult<(QueryBlocksResponse,)>;

    async fn transfer(args: TransferArgs) -> Result<BlockIndex, String>;
}

pub struct InterCanisterCallManager;

pub trait IcCdkSpawnManagerTrait {
    fn run<F: 'static + Future<Output = ()>>(future: F);
}

pub struct IcCdkSpawnManager;
