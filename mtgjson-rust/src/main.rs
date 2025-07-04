use clap::{Arg, Command};
use std::collections::HashSet;
use std::env;
use std::process;

mod classes;
mod providers;
mod builders;
mod compiled_classes;
mod constants;
mod utils_functions;

use builders::{OutputGenerator, PriceBuilder, build_mtgjson_set};
use providers::ScryfallProvider;
use classes::MtgjsonSetObject;

/// MTGJSON Main Executor - Rust equivalent of Python's __main__.py
#[derive(Debug, Clone)]
pub struct MtgjsonArgs {
    pub sets: Vec<String>,
    pub all_sets: bool,
    pub full_build: bool,
    pub resume_build: bool,
    pub compress: bool,
    pub pretty: bool,
    pub skip_sets: Vec<String>,
    pub price_build: bool,
    pub referrals: bool,
    pub no_alerts: bool,
    pub aws_ssm_download_config: Option<String>,
    pub aws_s3_upload_bucket: Option<String>,
    pub use_envvars: bool,
}

impl Default for MtgjsonArgs {
    fn default() -> Self {
        Self {
            sets: Vec::new(),
            all_sets: false,
            full_build: false,
            resume_build: false,
            compress: false,
            pretty: false,
            skip_sets: Vec::new(),
            price_build: false,
            referrals: false,
            no_alerts: false,
            aws_ssm_download_config: None,
            aws_s3_upload_bucket: None,
            use_envvars: false,
        }
    }
}

/// Parse command line arguments from user to determine how to spawn up
/// MTGJSON and complete the request.
pub fn parse_args() -> MtgjsonArgs {
    let app = Command::new("mtgjson5")
        .version("5.2.0")
        .author("MTGJSON Team")
        .about("MTGJSON - Magic: The Gathering JSON Data Generator")
        .arg(
            Arg::new("use-envvars")
                .long("use-envvars")
                .help("Use environment variables over parser flags for build operations")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("sets")
                .short('s')
                .long("sets")
                .help("Set(s) to build, using Scryfall set code notation. Non-existent sets silently ignored.")
                .value_name("SET")
                .num_args(0..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("all-sets")
                .short('a')
                .long("all-sets")
                .help("Build all possible sets, overriding the --sets option.")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("sets"),
        )
        .arg(
            Arg::new("full-build")
                .short('c')
                .long("full-build")
                .help("Build new prices, MTGSQLive, and compiled outputs like AllPrintings.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("resume-build")
                .short('x')
                .long("resume-build")
                .help("While determining what sets to build, ignore individual set files found in the output directory.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("compress")
                .short('z')
                .long("compress")
                .help("Compress the output folder's contents for distribution.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("When dumping JSON files, prettify the contents instead of minifying them.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("skip-sets")
                .long("skip-sets")
                .help("Purposely exclude sets from the build that may have been set using --sets or --all-sets.")
                .value_name("SET")
                .num_args(0..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("price-build")
                .long("price-build")
                .help("Build updated pricing data then exit.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("referrals")
                .short('R')
                .long("referrals")
                .help("Create and maintain a referral map for referral linkages.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-alerts")
                .long("no-alerts")
                .help("Prevent push notifications from sending when property keys are defined.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("aws-ssm-download-config")
                .long("aws-ssm-download-config")
                .help("AWS Parameter Store config name to load in, if local config file is not wanted/available.")
                .value_name("CONFIG_NAME")
                .num_args(1),
        )
        .arg(
            Arg::new("aws-s3-upload-bucket")
                .long("aws-s3-upload-bucket")
                .help("Upload finished results to an S3 bucket.")
                .value_name("BUCKET_NAME")
                .num_args(1),
        );

    let matches = app.get_matches();

    let mut args = MtgjsonArgs::default();

    if matches.get_flag("use-envvars") {
        args.use_envvars = true;
        println!("Using environment variables over parser flags");
        
        // Parse environment variables
        args.sets = env::var("SETS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_uppercase())
            .collect();
        
        args.all_sets = env::var("ALL_SETS").map(|v| v == "true").unwrap_or(false);
        args.full_build = env::var("FULL_BUILD").map(|v| v == "true").unwrap_or(false);
        args.resume_build = env::var("RESUME_BUILD").map(|v| v == "true").unwrap_or(false);
        args.compress = env::var("COMPRESS").map(|v| v == "true").unwrap_or(false);
        args.pretty = env::var("PRETTY").map(|v| v == "true").unwrap_or(false);
        
        args.skip_sets = env::var("SKIP_SETS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_uppercase())
            .collect();
        
        args.price_build = env::var("PRICE_BUILD").map(|v| v == "true").unwrap_or(false);
        args.referrals = env::var("REFERRALS").map(|v| v == "true").unwrap_or(false);
        args.no_alerts = env::var("NO_ALERTS").map(|v| v == "true").unwrap_or(false);
        args.aws_ssm_download_config = env::var("AWS_SSM_DOWNLOAD_CONFIG").ok();
        args.aws_s3_upload_bucket = env::var("AWS_S3_UPLOAD_BUCKET").ok();
    } else {
        // Parse command line arguments
        args.sets = matches
            .get_many::<String>("sets")
            .unwrap_or_default()
            .map(|s| s.to_uppercase())
            .collect();
        
        args.all_sets = matches.get_flag("all-sets");
        args.full_build = matches.get_flag("full-build");
        args.resume_build = matches.get_flag("resume-build");
        args.compress = matches.get_flag("compress");
        args.pretty = matches.get_flag("pretty");
        
        args.skip_sets = matches
            .get_many::<String>("skip-sets")
            .unwrap_or_default()
            .map(|s| s.to_uppercase())
            .collect();
        
        args.price_build = matches.get_flag("price-build");
        args.referrals = matches.get_flag("referrals");
        args.no_alerts = matches.get_flag("no-alerts");
        args.aws_ssm_download_config = matches.get_one::<String>("aws-ssm-download-config").cloned();
        args.aws_s3_upload_bucket = matches.get_one::<String>("aws-s3-upload-bucket").cloned();
    }

    args
}

/// Build each set one-by-one and output them to a file
pub fn build_mtgjson_sets(
    sets_to_build: &[String],
    output_pretty: bool,
    include_referrals: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Building {} Sets: {}", sets_to_build.len(), sets_to_build.join(", "));

    for set_to_build in sets_to_build {
        println!("Building set: {}", set_to_build);
        
        // Build the full set
        if let Some(mtgjson_set) = build_mtgjson_set(set_to_build) {
            // Handle referral components
            if include_referrals {
                // TODO: Implement referral building
                println!("Building referrals for {}", set_to_build);
            }

            // Dump set out to file
            let output_generator = OutputGenerator::new();
            output_generator.write_to_file(
                &mtgjson_set.get_windows_safe_set_code(),
                &mtgjson_set,
                output_pretty,
            )?;
        } else {
            eprintln!("Failed to build set: {}", set_to_build);
        }
    }

    if !sets_to_build.is_empty() && include_referrals {
        // TODO: Implement referral map fixup
        println!("Fixing up referral map");
    }

    Ok(())
}

/// MTGJSON Dispatcher - Main logic controller
pub fn dispatcher(args: MtgjsonArgs) -> Result<(), Box<dyn std::error::Error>> {
    // If a price build, simply build prices and exit
    if args.price_build {
        println!("Building prices...");
        let price_builder = PriceBuilder::default();
        let (_archive_prices, _today_prices) = price_builder.build_prices()?;
        
        let output_generator = OutputGenerator::new();
        output_generator.generate_compiled_prices_output(_archive_prices, _today_prices, args.pretty)?;
        
        if args.compress {
            // TODO: Implement compression
            println!("Compressing output...");
        }
        
        // TODO: Implement hash generation
        println!("Generating output file hashes...");
        return Ok(());
    }

    // Get sets to build
    let scryfall_provider = ScryfallProvider::new();
    let mut sets_to_build = scryfall_provider.get_sets_to_build(&args)?;
    
    if args.all_sets {
        // TODO: Load additional sets from local data
        println!("Loading additional sets from local data...");
        let additional_sets: HashSet<String> = HashSet::new(); // TODO: Implement
        let skip_sets: HashSet<String> = args.skip_sets.iter().cloned().collect();
        sets_to_build.extend(additional_sets.difference(&skip_sets).cloned());
    }

    if !sets_to_build.is_empty() {
        sets_to_build.sort();
        build_mtgjson_sets(&sets_to_build, args.pretty, args.referrals)?;
    }

    if args.full_build {
        println!("Generating compiled output files...");
        let output_generator = OutputGenerator::new();
        output_generator.generate_compiled_output_files(args.pretty)?;
        
        // TODO: Implement GitHubMTGSqliteProvider alternative formats
        println!("Building alternative formats...");
    }

    if args.compress {
        // TODO: Implement compression
        println!("Compressing MTGJSON contents...");
    }

    // TODO: Implement hash generation
    println!("Generating output file hashes...");

    if let Some(bucket) = args.aws_s3_upload_bucket {
        // TODO: Implement S3 upload
        println!("Uploading to S3 bucket: {}", bucket);
    }

    Ok(())
}

/// MTGJSON safe main call
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    let args = parse_args();

    // TODO: Initialize configuration
    if let Some(config_name) = &args.aws_ssm_download_config {
        println!("Loading AWS SSM config: {}", config_name);
        // TODO: Implement AWS SSM config loading
    } else {
        // TODO: Validate local config file exists
        println!("Using local configuration file");
    }

    println!("Starting MTGJSON v5.2.0 build");

    // TODO: Implement push notifications
    if !args.no_alerts {
        println!("Starting build with args: {:?}", args);
    }

    match dispatcher(args.clone()) {
        Ok(_) => {
            if !args.no_alerts {
                println!("Build finished successfully");
            }
        }
        Err(error) => {
            eprintln!("Exception caught: {}", error);
            if !args.no_alerts {
                eprintln!("Build failed: {}", error);
            }
            process::exit(1);
        }
    }

    Ok(())
}