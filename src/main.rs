use actix_web::{web, App, HttpResponse, HttpServer};
use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use log::{info, error};
use clap::{Command as ClapCommand, Arg};
use std::ffi::OsString;
use serde_derive::{Deserialize};
use std::fs;
use std::process::exit;
use toml;

struct EFMAPINodeStateArgs {
    config_file: String,
}

#[derive(Deserialize)]
struct EFMAPINodeStateData {
    config: EFMAPINodeStateConfig,
}

#[derive(Deserialize, Default)]
struct EFMAPINodeStateConfig {
    shell: String,
    primary_command: String,
    standby_command: String,
    listen_addr: String,
    port: u16,
    log_level: String,
}

impl EFMAPINodeStateArgs {
    fn new() -> Self {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // Basic app information
        let cmd = ClapCommand::new("efm-api-node-state")
            .version("0.2.0")
            .about("HTTP service and REST API exposing the state of the current EFM node.")
            .author("EDB");

        // Define the --config/-c command line option
        let config_option = Arg::new("config")
            .long("config") // allow --config
            .short('c') // allow -c
            .takes_value(true)
            .help("Configuration file path.")
            .required(true)
            .value_name("CONFIG_FILE");

        // Add in the arguments we want to parse
        let app = cmd.arg(config_option);

        // Extract the matches
        let matches = app.try_get_matches_from(args)?;

        // Extract the actual values
        let config_file = matches
            .value_of("config")
            .unwrap();

        Ok(EFMAPINodeStateArgs {
            config_file: config_file.to_string(),
        })
    }
}

async fn primary(primary_command: String, shell: String) -> HttpResponse {
    info!("/primary requested");

    exec_command(primary_command, shell).await
}

async fn standby(standby_command: String, shell: String) -> HttpResponse {
    info!("/standby requested");

    exec_command(standby_command, shell).await
}

async fn exec_command(command: String, shell: String) -> HttpResponse {
    info!("Executing the command: {}", command);

    let child = Command::new(shell)
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match child {
        Ok(mut child) => {
            // Read process' stdout and stderr and log them
            let mut stdout_lines = BufReader::new(child.stdout.take().unwrap()).lines();
            while let Some(line) = stdout_lines.next().await {
                match line {
                    Ok(line) => info!("stdout: {}", line),
                    Err(e) => error!("Unable to read process stdout ({})", e)
                }
            }
            let mut stderr_lines = BufReader::new(child.stderr.take().unwrap()).lines();
            while let Some(line) = stderr_lines.next().await {
                match line {
                    Ok(line) => info!("stderr: {}", line),
                    Err(e) => error!("Unable to read process stderr ({})", e)
                }
            }

            // Once we get process' status, meaning the execution is finished
            // we can return an HttpReponse based on the exit status code: if
            // the value is 0, everything is fine, if different from 0, then
            // we return an error.
            match child.status().await {
                Ok(exit_status) => {
                    match exit_status.code() {
                        Some(code) => {
                            if code == 0 {
                                info!("command exited with code 0");
                                HttpResponse::Ok()
                                    .content_type("application/json")
                                    .body("{\"message\":\"OK\"}")
                            } else {
                                info!("command exited with code {}", code);
                                HttpResponse::InternalServerError()
                                    .content_type("application/json")
                                    .body("{\"message\":\"KO\"}")
                            }
                        },
                        None => {
                            error!("failed to execute the command, empty status code");
                            HttpResponse::InternalServerError()
                                .content_type("application/json")
                                .body("{\"message\":\"KO\"}")
                        }
                    }
                },
                Err(e) => {
                    error!("failed to execute the command ({})", e);
                    HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .body("{\"message\":\"KO\"}")
                }
            }
        },
        Err(e) => {
            error!("failed to execute the command ({})", e);
            HttpResponse::InternalServerError()
                .content_type("application/json")
                .body("{\"message\":\"KO\"}")
        }
    }
}

async fn default() -> HttpResponse {
    error!("service not found");
    HttpResponse::NotFound()
        .content_type("application/json")
        .body("{\"message\":\"Service not found\"}")
}

fn load_config(config_path: String, config: &mut EFMAPINodeStateConfig) {
    // Read the configuration file
    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(e) => {
            error!("ERROR: could not read file {} ({})", config_path, e);
            eprintln!("ERROR: could not read file {} ({})", config_path, e);
            exit(1);
        }
    };

    // Parse TOML
    let data: EFMAPINodeStateData = match toml::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            error!("ERROR: unable to load data from {} ({})", config_path, e);
            eprintln!("ERROR: unable to load data from {} ({})", config_path, e);
            exit(1);
        }
    };

    // Move configuration values to the configuration structure
    config.shell = data.config.shell;
    config.primary_command = data.config.primary_command;
    config.standby_command = data.config.standby_command;
    config.listen_addr = data.config.listen_addr;
    config.port = data.config.port;
    config.log_level = data.config.log_level;
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize our config structure
    let mut config = EFMAPINodeStateConfig::default();

    // Parse command line arguments
    let args = EFMAPINodeStateArgs::new();
    // Load the configuration file
    load_config(args.config_file.to_string(), &mut config);

    // Logger initialization
    match config.log_level.as_str() {
        "DEBUG" => simple_logger::init_with_level(log::Level::Debug).unwrap(),
        "WARN"  => simple_logger::init_with_level(log::Level::Warn).unwrap(),
        "ERROR" => simple_logger::init_with_level(log::Level::Error).unwrap(),
        "INFO"  => simple_logger::init_with_level(log::Level::Info).unwrap(),
        _       => simple_logger::init_with_level(log::Level::Info).unwrap(),
    }

    // Move config.shell and config.command before passing them to the closure
    let shell = config.shell;
    let primary_command = config.primary_command;
    let standby_command = config.standby_command;

    // Let's start the HTTP server
    HttpServer::new(move || {
        let primary_command = primary_command.clone();
        let standby_command = standby_command.clone();
        let primary_shell = shell.clone();
        let standby_shell = shell.clone();
        App::new()
            .route("/primary", web::get().to(move ||
                primary(primary_command.to_string(), primary_shell.to_string())))
            .route("/standby", web::get().to(move ||
                standby(standby_command.to_string(), standby_shell.to_string())))
            .default_service(web::route().to(default))
    })
    .bind(format!("{}:{}", config.listen_addr, config.port))?
    .run()
    .await
}
