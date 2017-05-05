// (c) 2017 Joost Yervante Damad <joost@damad.be>

extern crate clap;
extern crate brewcalc;

use clap::{Arg, App, SubCommand};

use brewcalc::calc::*;

fn main() {
     let a = App::new("brewcalc")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .about("Calculations for brewing.")
        .subcommand(SubCommand::with_name("sg")
                    .about("calculate Specific Gravity")
                    .arg(Arg::with_name("PLATO")
                         .required(true)
                         .index(1)
                         .help("Plato or Brix")
                         )
        )
        .subcommand(SubCommand::with_name("plato")
                    .about("calculate Plato")
                    .arg(Arg::with_name("SG")
                         .required(true)
                         .index(1)
                         .help("Specific Gravity")
                         )
        );
    let m = a.get_matches();
    
    if let Some(sub_m) = m.subcommand_matches("sg") {
        let p = sub_m.value_of("PLATO").unwrap();
        let p = p.parse::<f64>().unwrap();
        let p = Plato(p);
        let s:Sg = p.into();
        let s = (s.0 * 1000.0).round()/1000.0;
        println!("{} Specific Gravity", s);
    }
    if let Some(sub_m) = m.subcommand_matches("plato") {
        let s = sub_m.value_of("SG").unwrap();
        let s = s.parse::<f64>().unwrap();
        let s = Sg(s);
        let p:Plato = s.into();
        println!("{:.1} Plato (or Brix)", p.0);
    }
}
