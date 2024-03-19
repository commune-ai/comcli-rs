use clap::Args;
use crate::global::{
    NETWORK,
    SUBNET
};

#[derive(Debug, Args)]
pub struct RegisterCommand {
    /// The module name
    pub name: String,

    /// The module address
    #[arg(default_value_t = String::from("NA"))]
    pub address: String,

    /// The initial stake amount
    #[arg(default_value_t = 0)]
    pub stake: u64,

    /// The subnet name
    #[arg(default_value_t = String::from(SUBNET))]
    pub subnet: String,

    /// The founder key
    #[arg(default_value_t = String::new())]
    pub key: String,

    /// The module key
    #[arg(default_value_t = String::new())]
    pub module_key: String,

    /// The network
    #[arg(default_value_t = String::from(NETWORK))]
    pub network: String,

    /// The flag to wait for inclusion
    #[arg(long, default_value_t = true)]
    pub wait_for_inclusion: bool,

    /// The flag to wait for finalization
    #[arg(long, default_value_t = true)]
    pub wait_for_finalization: bool,

    /// The existential balance
    #[arg(default_value_t = 1)]
    pub existential_balance: u64,

    /// The nonce
    #[arg(default_value_t = String::new())]
    pub nonce: String,

    /// The fmt
    #[arg(default_value_t = String::from("nano"))]
    pub fmt: String,
}

impl RegisterCommand {
    pub fn execute(&self) {
        println!("{:?}", self);
    }
}
