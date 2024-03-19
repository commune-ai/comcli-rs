use clap::Args;

#[derive(Debug, Args)]
pub struct StakeCommand {
    /// Module key which is staked on
    #[arg(default_value_t = String::new())]
    pub module: String,

    /// Stake amount
    #[arg(default_value_t = 0f64)]
    pub amount: f64,

    /// User key who stake on module
    #[arg(default_value_t = String::new())]
    pub key: String,

    /// The subnet netuid
    #[arg(default_value_t = 0)]
    pub netuid: u64,

    /// Network
    #[arg(default_value_t = String::new())]
    pub network: String,

    /// The existential deposit
    #[arg(default_value_t = 1f64)]
    pub existential_deposit: f64,

    /// The flag if update
    #[arg(long, default_value_t = false)]
    pub update: bool,
}

impl StakeCommand {
    pub fn execute(&self) {
        println!("{:?}", self);
    }
}
