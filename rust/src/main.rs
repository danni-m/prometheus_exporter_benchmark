// Will create an exporter with a single metric that will randomize the value
// of the metric everytime the exporter duration times out.

use env_logger::{
    Builder,
    Env,
};
use log::info;
use prometheus_exporter::prometheus::{register_gauge,opts,labels};
use rand::Rng;
use std::net::SocketAddr;
use std::{thread, time};
use clap::{Arg, App};


fn main() {
    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

        let matches = App::new("My Test Program")
        .version("0.1.0")
        .arg(Arg::with_name("num")
                 .short("n")
                 .long("number")
                 .takes_value(true)
                 .help("Number of counters"))
        .get_matches();

    // Parse address used to bind exporter to.
    let addr_raw = "0.0.0.0:8080";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter and update metrics every five seconds.
    let exporter = prometheus_exporter::start(addr).expect("can not start exporter");
    let duration = std::time::Duration::from_secs(5);

    // Create metric
    let random = register_gauge!(opts!("test_macro_counter_1", "help", labels!{"test" => "hello", "foo" => "bar",}))
        .expect("can not create gauge random_value_metric");

    let number_of_counters = matches.value_of("num").unwrap_or("2000").parse::<i32>().unwrap();
    for n in 1..number_of_counters {
        let number_label = format!("{}", n);
        let label2 = String::from("bar");
        let counter = register_gauge!(opts!(format!("counter{}", n), 
                                            "will set a random value", 
                                            labels!{"number" => &number_label , "foo" =>  &label2,})).expect("can not create gauge random_value_metric");
        counter.set(42.32);
    }


    loop {
        thread::sleep(time::Duration::from_millis(1000));
    }
}
