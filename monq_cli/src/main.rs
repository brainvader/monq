use anyhow::Context;
use clap::App;

use shared::{create_elasticsearch_client, get_es_url};
use shared::{log_info, setup_logger};

#[tokio::main]
async fn main() {
    setup_logger().with_context(|| "failed to setup logger")?;
    let url = get_es_url().with_context(|| "failed to get elasticsearch url")?;
    let client = create_elasticsearch_client(url)
        .with_context(|| "failed to create elasticsearch client")?;
    let setup_cmd = App::new("setup").about("Setup index for monq");
    App::new("monq")
        .version("0.1")
        .about("command line interface for monq")
        .author("BrainVader")
        .subcommand(setup_cmd)
        .get_matches();
}
