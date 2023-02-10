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

#[derive(Debug)]
struct Latlon {
    lat: f32,
    lon: f32,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct XML {
//     kml: kml,
// }

#[derive(Serialize, Deserialize, Debug)]
struct kml {
    Document: Doc,
}

#[derive(Serialize, Deserialize, Debug)]
struct Doc {
    name: String,
    Placemarks: Vec<PlaceMark>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlaceMark {
    name: String,
    LineString: Coordinates,
}

#[derive(Serialize, Deserialize, Debug)]
struct Coordinates {
    coordinates: Vec<Vec<f32>>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let query_company_name = "京王電鉄";
    // let query_line_name = "競馬場線";
    // let query_company_name = "東日本旅客鉄道";
    // let query_line_name = "山手線";
    // let mut latlons = Vec::new();

    let query_company_name = &args[1] as &str;
    let query_line_name = &args[2] as &str;

    let filename = format!("{}_{}.kml", query_company_name, query_line_name);
    println!("creating {} ...", filename);
    let mut file = File::create(filename)?;

    let content = std::fs::read_to_string("N02-20_RailroadSection.geojson").unwrap();
    let deserialized: Geo = serde_json::from_str(&content).unwrap();

    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
  <Document>
    <name>{}</name>"#,
        query_line_name
    );

    let mut id = 0;

    for feature in deserialized.features {
        if feature.properties.N02_003 == query_line_name
            && feature.properties.N02_004 == query_company_name
        {
            for line in &feature.geometry.coordinates {
                // if line.len() <= 2 {
                //     continue;
                // }

                write!(
                    file,
                    "
    <Placemark>
      <name>{}</name>
      <LineString>
        <coordinates>",
                    id
                );

                for coordinate in line {
                    assert_eq!(coordinate.len(), 2);
                    // let latlon = Latlon {
                    //     lat: coordinate[1],
                    //     lon: coordinate[0],
                    // };
                    // latlons.push(latlon);
                    write!(
                        file,
                        "
          {},{},0",
                        coordinate[0], coordinate[1]
                    );
                }

                write!(
                    file,
                    "
        </coordinates>
      </LineString>
    </Placemark>"
                );
            }
            id += 1;
        }
    }

    write!(
        file,
        r#"
  </Document>
</kml>
    "#
    );

    // for latlon in latlons {
    //     println!("          {},{},0", latlon.lon, latlon.lat);
    // }

    // let data = kml {
    //     Document: Doc {
    //         name: "query_line_name".to_string(),
    //         Placemarks: Vec::new(),
    //     },
    // };

    // let serialized = serde_xml_rs::to_string(&data).unwrap();
    // println!("serialized = {}", serialized);

    // writeln!(file, "{}", query_line_name);
    // writeln!(file, r#"{}"#, query_line_name);
    println!("succeeded!!");
    Ok(())
}
