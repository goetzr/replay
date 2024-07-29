use clap::{Command, Arg, value_parser};
use anyhow::bail;
use std::fmt;

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

    // TODO: Pick up here. Use new parse_float method.
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

struct F64Bound {
    value: f64,
    inclusive: bool,
}

impl F64Bound {
    fn new(value: f64, inclusive: bool) -> Self {
        Self { value, inclusive }
    }
}

struct F64Range {
    lbound: F64Bound,
    ubound: F64Bound,
}

impl F64Range {
    fn new(lbound: F64Bound, ubound: F64Bound) -> Self {
        Self { lbound, ubound }
    }

    fn contains(&self, value: f64) -> bool {
        let ok1 = match self.lbound {
            F64Bound { value: lbound, inclusive: true } => value >= lbound,
            F64Bound { value: lbound, inclusive: false } => value > lbound,
        };

        let ok2 = match self.ubound {
            F64Bound { value: ubound, inclusive: true } => value <= ubound,
            F64Bound { value: ubound, inclusive: false } => value < ubound,
        };

        ok1 && ok2
    }
}

impl fmt::Display for F64Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.lbound {
            F64Bound { value: lbound, inclusive: true } => write!(f, "[{lbound}, ")?,
            F64Bound { value: lbound, inclusive: false } => write!(f, "({lbound}, ")?,
        };
        match self.ubound {
            F64Bound { value: ubound, inclusive: true } => write!(f, "{ubound}], ")?,
            F64Bound { value: ubound, inclusive: false } => write!(f, "{ubound}), ")?,
        };
        Ok::<(), fmt::Error>(())
    }
}

fn parse_float(arg: &Arg, arg_value: &str, range: &F64Range) -> anyhow::Result<f64> {
    let arg_id = format!("'--{} <{}>'",
            arg.get_long().expect("argument must have a long option set"),
            arg.get_value_names().expect("argument must have a value name set")[0],
    );
    let Ok(arg_value) = arg_value.parse::<f64>() else {
        bail!("invalid value '{}' for {}: {} is not a valid floating point double",
            arg_value,
            arg_id,
            arg_value,
        );
    };
    if !range.contains(arg_value) {
        bail!("invalid value '{}' for {}: {} is not in the range {}",
            arg_value,
            arg_id,
            arg_value,
            range,
        );
    };

    Ok(arg_value)
}