use clap::Args;
use crate::global::NETWORK;

#[derive(Debug, Args)]
pub struct StatsCommand {
    /// The search key
    #[arg(default_value_t = String::new())]
    pub search: String,

    /// The subnet netuid
    #[arg(default_value_t = 0)]
    pub netuid: u64,

    /// The network
    #[arg(default_value_t = String::from(NETWORK))]
    pub network: String,

    /// The flag if df?
    #[arg(long, default_value_t = true)]
    pub df: bool,

    /// The flag if update
    #[arg(long, default_value_t = false)]
    pub update: bool,

    /// The flag if local
    #[arg(long, default_value_t = true)]
    pub local: bool,

    /// The cols
    #[arg(long, default_values_t = vec![
        "name".to_string(),
        "emission".to_string(),
        "incentive".to_string(),
        "dividends".to_string(),
        "stake".to_string(),
        "last_update".to_string(),
        "serving".to_string(),
    ])]
    pub cols: Vec<String>,

    /// The sort cols
    #[arg(long, default_values_t = vec![
        "name".to_string(),
        "serving".to_string(),
        "emission".to_string(),
        "stake".to_string(),
    ])]
    pub sort_cols: Vec<String>,

    /// The fmt
    #[arg(long, default_value_t = String::from("j"))]
    pub fmt: String,

    /// The flag if include_total
    #[arg(long, default_value_t = true)]
    pub include_total: bool,
}

impl StatsCommand {
    pub fn execute(&self) {
        println!("{:?}", self);
    }
}
