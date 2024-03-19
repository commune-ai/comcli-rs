use clap::{
    Parser, 
    Subcommand
};
use super::commands::{
    add_profit_shares::AddProfitSharesCommand,
    register::RegisterCommand, stake::StakeCommand,
    stats::StatsCommand, transfer::TransferCommand,
    unstake::UnstakeCommand,
    update_module::UpdateModuleCommand,
    update_subnet::UpdateSubnetCommand
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Command discriminator
    #[clap(subcommand)]
    pub command_type: CommandType
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    ///  Register Commune Module
    Register(RegisterCommand),

    ///  Show Commune Module Stats
    Stats(StatsCommand),

    ///  Transfer Com to other address
    Transfer(TransferCommand),

    ///  Stake on module
    Stake(StakeCommand),

    ///  Unstake on module
    Unstake(UnstakeCommand),

    ///  Add profit shares
    AddProfitShares(AddProfitSharesCommand),

    ///  Update Module
    UpdateModule(UpdateModuleCommand),

    ///  Update Subnet
    UpdateSubnet(UpdateSubnetCommand),
}
