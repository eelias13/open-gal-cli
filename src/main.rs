mod json_load;

use clap::{App, AppSettings, Arg};
use open_gal::{parse, to_jedec, CircuitConfig, TableData};

fn main() {
    let app = App::new("open-gal")
    .about("open-gal is a hardware description language for generic array logic chips (gal)")
    .version("0.1.0")
    .author("elias <maierelias13@gmail.com>")
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .subcommand(
        App::new("code2td")
            .about("converts the open-gal source code to the intermediate representation table data")
            .args(&[
                Arg::new("open-gal code")
                    .required(true)
                    .about("this is your open-gal sorce code"),
                Arg::new("table data json")
                    .required(true)
                    .about("is an  intermediate representation"),
                Arg::new("gal type")
                    .required(false)
                    .about("the path to your gal type json file"),    
                ]
            ),
    ).subcommand(
        App::new("td2jedec")
            .about("converts the intermediate representation table data to a jedec file")
            .args(&[
                Arg::new("table data json")
                    .required(true)
                    .about("is an  intermediate representation"),
                Arg::new("jedec filename")
                    .required(true)
                    .about("the name of your jedec file"),
                Arg::new("gal type")
                    .required(true)
                    .about("the path to your gal type json file"),    
                ]
            ),
    ).subcommand(
        App::new("code2jedec")
            .about("converts the open-gal source code to a jedec file")
            .args(&[
                Arg::new("open-gal code")
                    .required(true)
                    .about("this is your open-gal sorce code"),
                Arg::new("jedec filename")
                    .required(true)
                    .about("the name of your jedec file"),
                Arg::new("gal type")
                    .required(true)
                    .about("the path to your gal type json file"),
            ])
   ).subcommand(
    App::new("info")
        .about("shows information about a keyword")
    );

    let matches = app.get_matches();
    match matches.subcommand() {
        _ => (),
    }
    /*
    let table_data = get_table_data("./tableData.json").unwrap();
    let json = to_json(&table_data);
    let circuit_config = CircuitConfig::load("./Configs/g22v10.json").unwrap();
    println!("table_data: {:?}", table_data);
    println!("json: {}", json);
    */
}
