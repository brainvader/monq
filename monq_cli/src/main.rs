use clap::App;

#[tokio::main]
async fn main() {
    let setup_cmd = App::new("setup").about("Setup index for monq");
    App::new("monq")
        .version("0.1")
        .about("command line interface for monq")
        .author("BrainVader")
        .subcommand(setup_cmd)
        .get_matches();
}
