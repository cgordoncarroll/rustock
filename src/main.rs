extern crate clap;
mod stock;

use clap::{Arg, App, SubCommand};
use stock::Stock;

fn main() {
    let matches = App::new("Stock Grabber")
                    .version("0.0.1")
                    .about("Returns stock information for a given symbol")
                    .author("Gordon Carroll")
                    .arg(Arg::new("symbol")
                            .short("s")
                            .long("symbol")
                            .required(true)
                            .help("Stock ticker symbol")
                            .takes_value(true)).get_matches();
                            
    if let Some(sym) = matches.value_of("symbol") {
        let stock: Stock = Stock::new(sym.clone());
    }

}
