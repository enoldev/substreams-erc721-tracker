mod abi;
mod pb;
use abi::erc721::events::Transfer as ERC721TransferEvent;
use hex_literal::hex;
use pb::erc721::transfers::v1 as erc721;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;

const TRACKED_CONTRACT: [u8; 20] = hex!("8B5cC5E37C7fA12305669C3A7FC520663932b55D");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<erc721::Transfers, substreams::errors::Error> {
    let transfers: Vec<erc721::Transfer> = blk
        .events::<ERC721TransferEvent>(&[&TRACKED_CONTRACT])
        .map(|(event, _)| erc721::Transfer {
            from: Hex::encode(event.from),
            to: Hex::encode(event.to),
        })
        .collect();

    Ok(erc721::Transfers { transfers })
}
