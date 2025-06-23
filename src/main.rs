use geojson2kml::{error::Result, io::{load_geojson, StdinReader, StdoutWriter}, ui::InteractiveSession};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <geojson_file>", args[0]);
        eprintln!("Example: {} N02-20_RailroadSection.geojson", args[0]);
        std::process::exit(1);
    }
    
    let geojson_file = &args[1];
    let geo = load_geojson(geojson_file)?;
    
    let input = StdinReader;
    let output = StdoutWriter;
    let mut session = InteractiveSession::new(input, output);
    
    session.run(&geo)?;
    
    Ok(())
}