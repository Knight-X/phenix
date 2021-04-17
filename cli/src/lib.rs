use service;
use clap::App;

pub fn run() {
    let matches = App::new("captain")
        .version("0.0.1")
        .author("kazami")
        .subcommand(clap::App::new("run")
            .about("run the service")
            .version("0.0.1"))
        .get_matches();



    if let Some(ref matches) = matches.subcommand_matches("run") {
            service::run_service();
    }
}

