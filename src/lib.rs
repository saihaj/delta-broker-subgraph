mod abi;
mod pb;

use abi::contract::functions::Multicall;
use hex_literal::hex;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Function;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

const TRACKED_CONTRACT: [u8; 20] = hex!("74e95f3ec71372756a01eb9317864e3fdde1ac53");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn graph_out(blk: eth::Block) -> Result<EntityChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = EntityChangesTables::new();

    blk.calls().into_iter().for_each(|call| {
        if call.transaction.to == TRACKED_CONTRACT {
            if let Some(c) = Multicall::match_and_decode(&call) {
                let key = format!(
                    "{}{}",
                    Hex(&call.transaction.hash).to_string(),
                    call.transaction.index
                );

                tables
                    .create_row("multicall", key)
                    .set("tx_hash", &call.transaction.hash)
                    .set("from", &call.transaction.from)
                    .set("index", call.transaction.index)
                    .set("data", c.data);
            }
        }
    });

    Ok(tables.to_entity_changes())
}
