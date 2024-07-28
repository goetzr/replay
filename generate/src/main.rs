use clap::{Command, Arg, value_parser};

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

fn parse_args() -> u32 {
    let matches = Command::new("generate")
        .author("Russ Goetz, russgoetz@gmail.com")
        .version("0.0.1")
        .about("Generates a data file containing the flight of an aircraft in the airspace of an ASV.")
        .arg(
            Arg::new("duration")
                .long("duration")
                .value_parser(value_parser!(u32).range(..=3600))
                .required(true)
                .value_name("DURATION")
                .help("The duration, in seconds, of the flight, up to 3600 seconds (1 hour).")
        )
        .get_matches();

    *matches.get_one::<u32>("duration").unwrap()
}