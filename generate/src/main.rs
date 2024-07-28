use clap::{Command, Arg, value_parser};
use anyhow::bail;

fn main() {
    let duration = parse_args();
    println!("Duration = {duration} seconds.");
}

struct CliArgs {
    /// The duration of the flight, in seconds.
    duration: u32,
    /// The starting range of the aircraft from the radar, in meters.
    starting_range: u32,
    /// The starting azimuth of the aircraft from the radar, in degrees.
    starting_azimuth: f64,
    /// The speed of the aircraft, in m/s.
    speed: f64,
    /// The direction of travel of the aircraft, in degrees.
    direction: f64,
}

fn parse_args() -> anyhow::Result<CliArgs> {
    let min_duration = 1;
    let max_duration = 3600;
    let duration_arg = Arg::new("duration")
        .long("duration")
        .value_parser(value_parser!(u16).range(min_duration..=max_duration))
        .required(true)
        .value_name("DURATION")
        .help("The duration, in seconds, of the flight, in the rnage [1, 3600]."
    );

    let min_starting_range = 1;
    let max_starting_range = 500_000;
    let starting_range_arg = Arg::new("starting_range")
        .long("starting-range")
        .value_parser(value_parser!(u32).range(min_starting_range..=max_starting_range))
        .required(true)
        .value_name("STARTING-RANGE")
        .help("The starting range, in meters, of the aircraft from the radar, in the range [1, 500_000]."
    );

    let min_starting_azimuth = 0f64;
    let max_starting_azimuth = 360f64;
    let starting_azimuth_arg = Arg::new("starting_azimuth")
        .long("starting-azimuth")
        .required(true)
        .value_name("STARTING-AZIMUTH")
        .help("The starting azimuth, in degrees, of the aircraft from the radar, in the range [1, 360)."
    );
    
    let cmd = Command::new("generate")
        .author("Russ Goetz, russgoetz@gmail.com")
        .version("0.0.1")
        .about("Generates a data file containing the flight of an aircraft in the airspace of an ASV.")
        .arg(duration_arg)
        .arg(starting_range_arg)
        .arg(starting_azimuth_arg);
    let matches = cmd.get_matches();

    let duration = *matches.get_one::<u32>("duration").unwrap();
    let starting_range = *matches.get_one::<u32>("starting_range").unwrap();
    let starting_azimuth = matches.get_one::<&str>("starting_azimuth").unwrap();

    macro_rules! parse_f64 {
        ($arg_name:ident) => {
            let arg_var = concat_idents
        };
    }

    let starting_azimuth_id = format!("'--{} <{}>'",
            starting_azimuth_arg.get_long().unwrap(),
            starting_azimuth_arg.get_value_names().unwrap()[0],
    );
    let Ok(starting_azimuth) = starting_azimuth.parse::<f64>() else {
        bail!("invalid value '{}' for {}: {} is not a valid floating point double",
            starting_azimuth,
            starting_azimuth_id,
            starting_azimuth,
        );
    };
    if starting_azimuth < min_starting_azimuth || starting_azimuth >= max_starting_azimuth {
        bail!("invalid value '{}' for {}: {} is not in the range [{}, {})",
            starting_azimuth,
            starting_azimuth_id,
            starting_azimuth,
            min_starting_azimuth,
            max_starting_azimuth,
        );
    };
}