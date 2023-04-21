/*
    Appellation: xtask <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use xtask_sdk::Xtask;


#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    Xtask::default().init().run().await?;

    Ok(())
}


