use clap::{App, AppSettings, Arg, SubCommand};

#[derive(Debug)]
pub struct Args {
    pub verbose: bool,
    pub command: Option<Command>,
}

#[derive(Debug)]
pub enum Command {
    Write(Write),
    Send(Send),
}

impl Args {
    pub fn new() -> Args {
        Args::new_from(&mut std::env::args_os()).unwrap_or_else(|err| err.exit())
    }
    fn new_from<I, T>(args: I) -> Result<Args, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let args = Args::get_args_app().get_matches_from_safe(args)?;
        Ok(Args {
            verbose: args.occurrences_of("v") > 0,
            command: Args::get_config_from_cl(args),
        })
    }

    fn get_args_app<'a, 'b>() -> App<'a, 'b> {
        App::new("bulkmail")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Giovanni Bassi <giggio@giggio.net>")
            .about("Creates mail messages from a csv file and a template file and sends them.")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(
                Arg::with_name("v")
                    .short("v")
                    .long("verbose")
                    .global(true)
                    .multiple(true)
                    .help("Sets the level of verbosity"),
            )
            .subcommand(
                SubCommand::with_name("send")
                    .about("Sends an e-mail message if the average of the last measurements is bellow a bandwith value")
                    .arg(
                        Arg::with_name("csv_file_path")
                            .takes_value(true)
                            .index(1)
                            .required(true)
                            .help("File path of the csv file to use"),
                    )
                    .arg(
                        Arg::with_name("subject")
                            .takes_value(true)
                            .index(2)
                            .required(true)
                            .help("Subject to use with all e-mail messages"),
                    )
                    .arg(
                        Arg::with_name("template_file_path")
                            .takes_value(true)
                            .index(3)
                            .required(true)
                            .help("File path of the message template (Handlebars file)"),
                    )
                    .arg(
                        Arg::with_name("sender email")
                            .takes_value(true)
                            .index(4)
                            .required(true)
                            .help("E-mail address to send the message from"),
                    )
                    .arg(
                        Arg::with_name("smtp server")
                            .long("smtp")
                            .takes_value(true)
                            .index(5)
                            .required(true)
                            .help("SMTP server and port to use, use server:port")
                            .validator(|server_and_port| {
                                let parts: Vec<&str> = server_and_port.split(':').collect();
                                if parts.len() != 2 {
                                    return Err("Not valid server".to_owned());
                                }
                                if parts[1].parse::<u16>().is_err() {
                                    return Err("Port is not in the correct format.".to_owned());
                                }
                                Ok(())
                            }),
                    )
                    .arg(
                        Arg::with_name("simulate")
                            .short("s")
                            .long("simulate")
                            .help("Should write e-mail to stdout instead of sending e-mail"),
                    )
                    .arg(
                        Arg::with_name("username")
                            .short("u")
                            .long("username")
                            .takes_value(true)
                            .requires("password")
                            .help("SMTP server user for authentication"),
                    )
                    .arg(
                        Arg::with_name("password")
                            .short("p")
                            .long("password")
                            .takes_value(true)
                            .help("SMTP server password for authentication"),
                    ),
            )
            .subcommand(
                SubCommand::with_name("write")
                    .about("Writes the e-mails to a file.")
                    .arg(
                        Arg::with_name("csv_file_path")
                            .takes_value(true)
                            .index(1)
                            .required(true)
                            .help("File path of the csv file to use"),
                    )
                    .arg(
                        Arg::with_name("subject")
                            .takes_value(true)
                            .index(2)
                            .required(true)
                            .help("Subject to use with all e-mail messages"),
                    )
                    .arg(
                        Arg::with_name("template_file_path")
                            .takes_value(true)
                            .index(3)
                            .required(true)
                            .help("File path of the message template (Handlebars file)"),
                    )
                    .arg(
                        Arg::with_name("output_file_path")
                            .takes_value(true)
                            .index(4)
                            .required(true)
                            .help("File path of output file"),
                    )
                    .arg(
                        Arg::with_name("simulate")
                            .short("s")
                            .long("simulate")
                            .help("Should simulate files reads and writes instead of using actual files."),
                    ),
            )
    }

    fn get_smtp_from_cl(args: &clap::ArgMatches) -> Option<Smtp> {
        let smtp = if let Some(server_and_port) = args.value_of("smtp server") {
            let parts: Vec<&str> = server_and_port.split(':').collect();
            let server = parts[0];
            let port = parts[1].parse::<u16>().ok()?;
            let credentials;
            if let Some(password) = args.value_of("password") {
                credentials = Some(Credentials {
                    username: args.value_of("username").map(|u| u.to_owned()),
                    password: password.to_owned(),
                });
            } else {
                credentials = None;
            }
            let email = args.value_of("sender email")?;
            Some(Smtp {
                email: email.to_owned(),
                server: server.to_owned(),
                port,
                credentials,
            })
        } else {
            None
        };
        smtp
    }

    fn get_config_from_cl(args: clap::ArgMatches) -> Option<Command> {
        match args.subcommand() {
            ("write", Some(write_args)) => Some(Command::Write(Write {
                simulate: write_args.is_present("simulate"),
                file_path: write_args.value_of("csv_file_path").unwrap().to_owned(),
                output_file_path: write_args.value_of("output_file_path").unwrap().to_owned(),
                subject: write_args.value_of("subject").unwrap().to_owned(),
                template_file_path: write_args
                    .value_of("template_file_path")
                    .unwrap()
                    .to_owned(),
            })),
            ("send", Some(send_args)) => Some(Command::Send(Send {
                simulate: send_args.is_present("simulate"),
                file_path: send_args.value_of("csv_file_path").unwrap().to_owned(),
                subject: send_args.value_of("subject").unwrap().to_owned(),
                template_file_path: send_args.value_of("template_file_path").unwrap().to_owned(),
                smtp: Args::get_smtp_from_cl(send_args).unwrap(),
            })),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Write {
    pub simulate: bool,
    pub file_path: String,
    pub subject: String,
    pub template_file_path: String,
    pub output_file_path: String,
}

#[derive(Debug)]
pub struct Send {
    pub simulate: bool,
    pub file_path: String,
    pub subject: String,
    pub template_file_path: String,
    pub smtp: Smtp,
}

#[derive(Debug)]
pub struct Smtp {
    pub server: String,
    pub email: String,
    pub port: u16,
    pub credentials: Option<Credentials>,
}

#[derive(Debug)]
pub struct Credentials {
    pub username: Option<String>,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn args_run_simulated() {
        let write = match Args::new_from(
            [
                "bulkmail",
                "write",
                "/tmp/in.csv",
                "A subject",
                "/tmp/in.handlebars",
                "/tmp/output.txt",
                "--simulate",
            ]
            .iter(),
        )
        .unwrap()
        .command
        .unwrap()
        {
            Command::Send(_) => panic!("Should not be send"),
            Command::Write(write) => write,
        };
        assert!(write.simulate);
    }
}
