use stock_metrics_driver::{
    startup::{init_app, startup},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    // let modules = Modules::new().await;
    let _ = startup().await;

    Ok(())
}
