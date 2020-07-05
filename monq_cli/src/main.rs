use clap::App;

#[tokio::main]
async fn main() {
    App::new("monq")
        .version("0.1")
        .about("command line interface for monq")
        .author("BrainVader")
        .get_matches();
}
