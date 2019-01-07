extern crate alqhemist;
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Alqhemist")
        .version("0.1.0")
        .author("Nurahmadie <nurahmadie@gmail.com>")
        .arg(
            Arg::with_name("schema")
                .short("s")
                .value_name("FILE")
                .help("Specify schema to convert."),
        )
        .arg(
            Arg::with_name("framework")
                .short("f")
                .value_name("name")
                .help("Specify which framework this schema should be converted to."),
        )
        .get_matches();

    let schema = matches.value_of("schema").unwrap_or("./schema.graphql");
    let _framework = matches.value_of("framework").unwrap_or("ruby_graphql");

    alqhemist::transmute(schema).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        std::process::exit(-1);
    });
}
