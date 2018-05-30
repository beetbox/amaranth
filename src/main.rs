extern crate clap;
extern crate reqwest;
extern crate url;

use url::Url;

fn cmd_list(endpoint: &str, album: bool) {
    let base_url = Url::parse(endpoint).unwrap();
    let tracks_url = base_url.join("/aura/tracks").unwrap();
    let res = reqwest::get(tracks_url.as_str()).unwrap().text().unwrap();
    println!("{} {}", album, res);
}

fn main() {
    let list = clap::SubCommand::with_name("list")
        .about("print descriptions of music")
        .arg(clap::Arg::with_name("album")
             .short("a")
             .long("album")
             .help("list albums instead of tracks"))
        .arg(clap::Arg::with_name("query")
             .index(1)
             .multiple(true)
             .help("query to match music"));

    let app = clap::App::new("amaranth")
        .version("1.0.0")
        .arg(clap::Arg::with_name("url")
             .short("u")
             .default_value("http://localhost:8337")
             .help("server URL to connect to"))
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(list);

    let matches = app.get_matches();
    let url = matches.value_of("url").unwrap();
    if let Some(matches) = matches.subcommand_matches("list") {
        let album = matches.is_present("album");
        let num_parts = if let Some(query_parts) = matches.values_of("query") {
            query_parts.count()
        } else {
            0
        };
        println!("list! {} {} {}", url, num_parts, album);
        cmd_list(url, album);
    } else {
        assert!(false);
    }
}
