use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ERC721Transfer", "abi/erc721.abi.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;

    Ok(())
}
