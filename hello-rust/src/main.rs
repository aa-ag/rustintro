use structopt::StructOpt;

#[derive(StructOpt)]
struct  Cli {
    city: String,
    state: String,
}

struct Forecast {
    timezone: i32,
    id: i32,
    weather: Weather,
}

struct Weather {
    details: Details
}

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