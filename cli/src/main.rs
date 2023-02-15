use clap::Parser;
use ityfuzz::config::{Config, FuzzerTypes};
use ityfuzz::contract_utils::ContractLoader;
use ityfuzz::fuzzers::basic_fuzzer;
use ityfuzz::fuzzers::cmp_fuzzer::cmp_fuzzer;
use ityfuzz::fuzzers::df_fuzzer::df_fuzzer;
use ityfuzz::onchain::endpoints::OnChainConfig;
use std::path::PathBuf;

/// CLI for ItyFuzz
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Glob pattern to find contracts
    #[arg(short, long)]
    contract_glob: String,

    /// target single contract -- Optional
    #[arg(short, long)]
    target_contract: Option<String>,

    /// Fuzzer type -- Optional
    #[arg(short, long)]
    fuzzer_type: Option<String>,

    /// Enable onchain
    #[arg(short, long)]
    onchain: Option<bool>,

    /// Onchain endpoint URL
    #[arg(long)]
    onchain_url: Option<String>,

    /// Onchain chain ID
    #[arg(long)]
    onchain_chain_id: Option<u32>,

    /// Onchain block number
    #[arg(long)]
    onchain_block_number: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        onchain: if args.onchain.is_some() && args.onchain.unwrap() {
            Some(OnChainConfig::new(
                args.onchain_url
                    .unwrap_or("https://bsc-dataseed1.binance.org/".to_string()),
                args.onchain_chain_id.unwrap_or(56),
                args.onchain_block_number.unwrap_or(0),
            ))
        } else {
            None
        },
        fuzzer_type: FuzzerTypes::from_str(args.fuzzer_type.unwrap_or("cmp".to_string()).as_str())
            .expect("unknown fuzzer"),
        contract_info: if args.target_contract.is_none() {
            ContractLoader::from_glob(args.contract_glob.as_str()).contracts
        } else {
            ContractLoader::from_glob_target(
                args.contract_glob.as_str(),
                args.target_contract.unwrap().as_str(),
            )
            .contracts
        },
        oracle: None,
    };

    match config.fuzzer_type {
        FuzzerTypes::CMP => cmp_fuzzer(config),
        FuzzerTypes::DATAFLOW => df_fuzzer(config),
        // FuzzerTypes::BASIC => basic_fuzzer(config)
        _ => {}
    }
    //
    //     Some(v) => {
    //         match v.as_str() {
    //             "cmp" => {
    //                 cmp_fuzzer(&String::from(args.contract_glob), args.target_contract);
    //             }
    //             "df" => {
    //                 df_fuzzer(&String::from(args.contract_glob), args.target_contract);
    //             }
    //             _ => {
    //                 println!("Fuzzer type not supported");
    //             }
    //         }
    //     },
    //     _ => {
    //         df_fuzzer(&String::from(args.contract_glob), args.target_contract);
    //     }
    // }
}
