extern crate clap;

fn main() {
    let list = clap::SubCommand::with_name("list")
        .about("print descriptions of music")
        .arg(clap::Arg::with_name("album")
             .short("a")
             .help("list albums instead of tracks"))
        .arg(clap::Arg::with_name("query")
             .index(1)
             .multiple(true)
             .help("query to match music"));

    clap::App::new("amaranth")
        .version("1.0.0")
        .arg(clap::Arg::with_name("url")
             .short("u")
             .default_value("http://localhost:8337")
             .help("server URL to connect to"))
        .subcommand(list)
        .get_matches();
}
