use clap::Args;

#[derive(Debug, Args)]
pub struct TransferCommand {
    /// The destination address
    pub dest: String,

    /// Transfer amount
    pub amount: f64,

    /// Key
    #[arg(default_value_t = String::new())]
    pub key: String,

    /// Network
    #[arg(default_value_t = String::new())]
    pub network: String,

    /// Nonce
    #[arg(default_value_t = String::new())]
    pub nonce: String,
}

impl TransferCommand {
    pub fn execute(&self) {
        println!("{:?}", self);
    }
}
