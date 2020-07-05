use anyhow::Context;
use clap::App;

use shared::{create_elasticsearch_client, get_es_url};
use shared::{log_info, setup_logger};

async fn setup() -> anyhow::Result<()> {
    log_info(&format!("{}", "setup monq"));
    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger().with_context(|| "failed to setup logger")?;
    let url = get_es_url().with_context(|| "failed to get elasticsearch url")?;
    let client = create_elasticsearch_client(url)
        .with_context(|| "failed to create elasticsearch client")?;
    let mut rt =
        tokio::runtime::Runtime::new().with_context(|| "failed to create tokio runtime")?;

    let setup_cmd = App::new("setup").about("Setup index for monq");
    let monq_cmd = App::new("monq")
        .version("0.1")
        .about("command line interface for monq")
        .author("BrainVader")
        .subcommand(setup_cmd);

    let matches = monq_cmd.get_matches();

    Ok(())
}
