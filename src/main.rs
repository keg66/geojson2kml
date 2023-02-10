use core::panic;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Geo {
    r#type: String,
    name: String,
    crs: Crs,
    features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Crs {
    r#type: String,
    properties: Property,
}

#[derive(Serialize, Deserialize, Debug)]
struct Property {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Feature {
    r#type: String,
    properties: GeoProperty,
    geometry: Geometry,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct GeoProperty {
    N02_001: String,
    N02_002: String,
    N02_003: String,
    N02_004: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Geometry {
    r#type: String,
    coordinates: Vec<Vec<Vec<f32>>>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let query_company_name = &args[1] as &str;
    let query_line_name = &args[2] as &str;

    let filename = format!("{}_{}.kml", query_company_name, query_line_name);
    println!("creating {} ...", filename);
    let mut file = File::create(&filename)?;

    let content = std::fs::read_to_string("N02-20_RailroadSection.geojson").unwrap();
    let deserialized: Geo = serde_json::from_str(&content).unwrap();

    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
  <Document>
    <name>{} {}</name>"#,
        query_company_name, query_line_name
    )?;

    let mut id = 0;

    for feature in deserialized.features {
        if feature.properties.N02_003.contains(query_line_name)
            && feature.properties.N02_004.contains(query_company_name)
        {
            for line in &feature.geometry.coordinates {
                write!(
                    file,
                    "
    <Placemark>
      <name>{}</name>
      <LineString>
        <coordinates>",
                    id
                )?;

                for coordinate in line {
                    assert_eq!(coordinate.len(), 2);
                    write!(
                        file,
                        "
          {},{},0",
                        coordinate[0], coordinate[1]
                    )?;
                }

                write!(
                    file,
                    "
        </coordinates>
      </LineString>
    </Placemark>"
                )?;
            }
            id += 1;
        }
    }

    if id == 0 {
        drop(file);
        std::fs::remove_file(filename)?;
        panic!("{} {} is not found...", query_company_name, query_line_name);
    }

    write!(
        file,
        r#"
  </Document>
</kml>
    "#
    )?;

    println!("succeeded!!");
    Ok(())
}
