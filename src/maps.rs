use crate::abi::{self};
use crate::pb::erc20::types::v1::{
    ApprovalEvent, Block as Erc20Block, TransferEvent,
};
use abi::erc20::events::{Approval, Transfer};
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Erc20Block, Error> {
    let (approvals, transfers) = map_events(&block);

    Ok(Erc20Block {
        approvals,
        transfers
    })
}

pub fn map_events(block: &Block) -> (Vec<ApprovalEvent>, Vec<TransferEvent>) {
    let mut approvals = vec![];
    let mut transfers = vec![];

    for log in block.logs() {
        // received logs are only from successful transaction, no need to check
        // filter by type
        if let Some(approval) = Approval::match_and_decode(log.log) {
            approvals.push(decode_approval(approval, log));
            continue;
        }

        if let Some(transfer) = Transfer::match_and_decode(log.log) {
            transfers.push(decode_transfer(transfer, log));
            continue;
        }

        // no data
    }

    (approvals, transfers)
}

fn decode_transfer(event: Transfer, log: LogView) -> TransferEvent {
    TransferEvent {
        // contract address
        address: Hex::encode(log.address()),

        // event payload
        from: Hex::encode(event.from),
        to: Hex::encode(event.to),
        value: event.value.to_string(),

        // trace information
        transaction: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
    }
}

fn decode_approval(event: Approval, log: LogView) -> ApprovalEvent {
    ApprovalEvent {
        // contract address
        address: Hex::encode(log.address()),

        // event payload
        owner: Hex::encode(event.owner),
        spender: Hex::encode(event.spender),
        value: event.value.to_string(),

        // trace information
        transaction: Hex::encode(&log.receipt.transaction.hash),
        block_index: log.log.block_index.into(),
    }
}
