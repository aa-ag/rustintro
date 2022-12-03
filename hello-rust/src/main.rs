use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde_derive::{Deserialize};

#[derive(StructOpt)]
struct  Cli {
    city: String,
    state: String,
}

#[derive(Deserialize,Debug)]
struct Forecast {
    timezone: i32,
    id: i32,
    weather: Weather,
}

#[derive(Deserialize,Debug)]
struct Weather {
    details: Details
}

#[derive(Deserialize,Debug)]
struct  Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

fn main() {
    let args = Cli::from_args();
    println!("Hello from the city of {}, {}.", args.city, args.state)
}