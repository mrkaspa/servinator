use clap::{Arg, App};

pub enum Command {
    Run(Data),
}

#[derive(Debug)]
pub struct Data {
    pub dir: String,
    pub config: Option<String>,
}

pub fn process_cli() -> Command {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the input file to use")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .value_name("FILE")
                 .help("Sets a custom config file")
                 .takes_value(true))
        .get_matches();

    let config = matches.value_of("config").map(String::from);
    let dir = matches
        .value_of("INPUT")
        .expect("Please provide the dir");

    Command::Run(Data {
                     config: config,
                     dir: String::from(dir),
                 })
}
