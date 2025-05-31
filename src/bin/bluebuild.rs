use blue_build::commands::{BlueBuildArgs, BlueBuildCommand, CommandArgs};
use blue_build_process_management::{logging::Logger, signal_handler};
use clap::Parser;
use log::LevelFilter;

fn main() {
    let args = BlueBuildArgs::parse();

    Logger::new()
        .filter_level(args.verbosity.log_level_filter())
        .filter_modules([
            ("hyper::proto", LevelFilter::Off),
            ("hyper_util", LevelFilter::Off),
            ("oci_distribution", LevelFilter::Off),
            ("reqwest", LevelFilter::Off),
            ("oci_client", LevelFilter::Off),
            ("rustls", LevelFilter::Off),
        ])
        .log_out_dir(args.log_out.clone())
        .init();
    log::trace!("Parsed arguments: {args:#?}");

    signal_handler::init(|| match args.command {
        CommandArgs::Build(mut command) => command.run(),
        CommandArgs::Generate(mut command) => command.run(),
        CommandArgs::Switch(mut command) => command.run(),
        CommandArgs::Login(mut command) => command.run(),
        CommandArgs::New(mut command) => command.run(),
        CommandArgs::Init(mut command) => command.run(),
        CommandArgs::GenerateIso(mut command) => command.run(),
        CommandArgs::Validate(mut command) => command.run(),
        CommandArgs::Prune(mut command) => command.run(),
        CommandArgs::BugReport(mut command) => command.run(),
        CommandArgs::Completions(mut command) => command.run(),
    });
}
