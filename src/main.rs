use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashSet;
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

#[derive(Hash, Eq, PartialEq)]
struct TrainLine<'a> {
    company_name: &'a str,
    line_name: &'a str,
}

fn get_string_from_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn search_candidates<'a>(query_line_name: &str, geo: &'a Geo) -> HashSet<TrainLine<'a>> {
    let mut candidates = HashSet::new();

    for feature in &geo.features {
        if feature.properties.N02_003.contains(query_line_name) {
            let candidate = TrainLine {
                company_name: &feature.properties.N02_004,
                line_name: &feature.properties.N02_003,
            };
            candidates.insert(candidate);
        }
    }

    candidates
}

fn choose_id<'a>(query_line_name: &str, candidates: &'a Vec<TrainLine<'a>>) -> Option<usize> {
    let candidate_num = candidates.len();
    if candidate_num == 1 {
        return Some(0);
    } else if candidate_num > 1 {
        println!("candidates:");
        for id in 0..candidate_num {
            println!(
                "[{}] {} {}",
                id, &candidates[id].company_name, &candidates[id].line_name
            );
        }
        println!("choose: ");

        loop {
            let answer = get_string_from_stdin();

            match answer.parse::<usize>() {
                Ok(chosen_id) => {
                    if chosen_id >= candidate_num {
                        eprintln!("{} is invalid... choose again:", answer);
                        continue;
                    }
                    return Some(chosen_id);
                }
                Err(_) => {
                    eprintln!("{} is invalid... choose again:", answer);
                    continue;
                }
            }
        }
    }
    eprintln!("{} is not found...", query_line_name);
    None
}

fn generate_kml(train_line: &TrainLine, geo: &Geo) -> std::io::Result<()> {
    let company_name = train_line.company_name;
    let line_name = train_line.line_name;

    let filename = format!("{}_{}.kml", company_name, line_name);
    println!("creating {} ...", filename);
    let mut file = File::create(&filename)?;

    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>
<name>{} {}</name>"#,
        company_name, line_name
    )?;

    let mut id = 0;

    for feature in &geo.features {
        if feature.properties.N02_003 == line_name && feature.properties.N02_004 == company_name {
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

fn main() {
    let content = std::fs::read_to_string("N02-20_RailroadSection.geojson").unwrap();
    let geo: Geo = serde_json::from_str(&content).unwrap();

    loop {
        println!("enter train line name:");
        let query = get_string_from_stdin();
        let query_line_name = &query as &str;

        let candidates: Vec<TrainLine> = search_candidates(query_line_name, &geo)
            .into_iter()
            .collect();

        let chosen_id = choose_id(query_line_name, &candidates);

        if let Some(id) = chosen_id {
            if let Err(_) = generate_kml(&candidates[id], &geo) {
                eprintln!("failed to generate kml ...");
            }
        }
    }
}
