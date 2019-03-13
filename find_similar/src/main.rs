use clap::{Arg, App, SubCommand};
use std::fs::File;
use std::io::Read;
use geohash::{encode, decode, neighbor, Direction, Coordinate};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use find_similar::storage::Geohash;

const GEOHASH_LENGTH:usize = 11;

fn main() {
    let matches = App::new("Coincidence finder")
        .version("1.0")
        .author("Abraham Toriz <categulario@gmail.com>")
        .about("Finds coincidence in multiple gps tracks")
        .arg(Arg::with_name("radius")
             .short("r")
             .long("radius")
             .help("Radius of coincidence for gps points")
             .takes_value(true)
             .default_value("50")
             .required(false))
        .arg(Arg::with_name("INPUT")
           .help("Input files or directories")
           .takes_value(true)
           .multiple(true))
        .get_matches();

    let filenames: Vec<_> = matches.values_of("INPUT").unwrap().collect();

    for filename in filenames {
        let mut file = File::open(filename)
            .expect(&format!("Could not open {}", filename));
        let mut contents = String::new();

        file.read_to_string(&mut contents).expect("Could not read the file");

        let json_data = json::parse(&contents).expect("File is not valid json");

        let coordinates = &json_data["features"][0]["geometry"]["coordinates"];
        let mut storage = HashMap::new();

        for coord in coordinates.members() {
            let coord = Coordinate{
                x: coord[0].as_f64().expect("malformed geojson"),
                y: coord[1].as_f64().expect("malformed geojson"),
            };
            let hash = encode(coord, GEOHASH_LENGTH).unwrap();

            match storage.entry(hash) {
                Entry::Occupied(oe) => {
                    println!("Repeaded key: {}", oe.key());
                },
                Entry::Vacant(ve) => {
                    ve.insert(1);
                },
            }
        }
    }
}
