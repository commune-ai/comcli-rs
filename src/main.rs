mod cli;
pub mod global;

use cli::top_level::{Cli, CommandType};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    
    match args.command_type {
        CommandType::Register(register_command) => register_command.execute(),
        CommandType::Stats(stats_command) => stats_command.execute(),
        CommandType::Transfer(transfer_command) => transfer_command.execute(),
        CommandType::Stake(stake_command) => stake_command.execute(),
        CommandType::Unstake(unstake_command) => unstake_command.execute(),
        CommandType::AddProfitShares(add_profit_shares_command) => add_profit_shares_command.execute(),
        CommandType::UpdateModule(update_module_command) => update_module_command.execute(),
        CommandType::UpdateSubnet(update_subnet_command) => update_subnet_command.execute(),
        _ => panic!("something missing"),
    }
}
