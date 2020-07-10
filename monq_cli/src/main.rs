use anyhow::Context;
use clap::{App, Arg};

use shared::es::api::create_elasticsearch_client;
use shared::es::api::{create_index, delete_monq, post_doc};
use shared::run_docker_compose;
use shared::{get_es_url, log_info, setup_logger};

async fn setup() -> anyhow::Result<()> {
    let url = get_es_url().with_context(|| "failed to get elasticsearch url")?;
    let client = create_elasticsearch_client(url)
        .with_context(|| "failed to create elasticsearch client")?;
    {
        let response_body = delete_monq(&client)
            .await
            .with_context(|| "failed to delete monq")?;
        log_info(&format!("Delete: {}", response_body));
    }
    let response_body = create_index(&client, "./elasticsearch/index.json")
        .await
        .with_context(|| "failed to create monq")?;
    log_info(&format!("{}", response_body));
    Ok(())
}

async fn seed(doc: &str) -> anyhow::Result<()> {
    let url = get_es_url().with_context(|| "failed to get elasticsearch url")?;
    let client = create_elasticsearch_client(url)
        .with_context(|| "failed to create elasticsearch client")?;
    let _ = post_doc(&client, doc)
        .await
        .with_context(|| "faild to post doc")?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    setup_logger().with_context(|| "failed to setup logger")?;
    let status = run_docker_compose()?;
    if status.success() {
        let mut rt =
            tokio::runtime::Runtime::new().with_context(|| "failed to create tokio runtime")?;

        let setup_cmd = App::new("setup").about("Setup index for monq");
        let seed_cmd = App::new("seed").about("Seed intial data").arg(
            Arg::with_name("document")
                .long("doc")
                .short("d")
                .required(true)
                .takes_value(true)
                .help("specify document file"),
        );
        let monq_cmd = App::new("monq")
            .version("0.1")
            .about("command line interface for monq")
            .author("BrainVader")
            .subcommand(setup_cmd)
            .subcommand(seed_cmd);

        let matches = monq_cmd.get_matches();

        match matches.subcommand_name() {
            Some("setup") => rt.block_on(setup())?,
            None => log_info(&"No Subcommand Provided"),
            _ => unreachable!(),
        }
    }
    Ok(())
}
