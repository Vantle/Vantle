#[derive(clap::Args)]
pub struct Argument {
    #[arg(
        long,
        help = "Sink endpoints (e.g., log:///tmp/trace.jsonl, chrome:///tmp/trace.json, grpc://127.0.0.1:50051)"
    )]
    pub sink: Vec<String>,
}
