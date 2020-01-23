use std::error::Error;

pub struct Config {
    pub listen_port: u16,
}

impl Config {
    pub fn parse_cmd_line() -> Result<Config, Box<dyn Error>> {
        use clap::{App, Arg};

        let app = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(Arg::with_name("listen_port")
                .help("listening port number")
                .short("p")
                .long("port")
                .takes_value(true)
        );

        let matches = app.get_matches();
        let listen_port = matches.value_of("listen_port")
            .unwrap_or("8080")
            .parse()?;

        Ok(Config { listen_port })
    }
}