use clap::{App, Arg};
use signaling_server::server::Server;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("ksig")
        .version("0.0.1")
        .author("Ko")
        .about("Signaling Server")
        .arg(
            Arg::with_name("LISTEN_ADDRESS")
                .short("l")
                .long("listen")
                .help("listen address")
                .default_value("127.0.0.1:8081")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("WORKER_NUMBER")
                .short("w")
                .long("worker_number")
                .help("Worker Running Number")
                .default_value("1")
                .takes_value(true),
        )
        .get_matches();

    let listen = matches.value_of("LISTEN_ADDRESS").unwrap();
    let workers = matches.value_of("WORKER_NUMBER").unwrap();
    println!("running server.");
    Server::new((listen, workers.parse::<usize>().unwrap()))
        .listen()
        .await
}
