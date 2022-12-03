use structopt::StructOpt;

#[derive(StructOpt)]
struct  Cli {
    city: String,
    state: String,
}

fn main() {
    let args = Cli::from_args();
    println!("Hello from the city of {}, {}.", args.city, args.state)
}