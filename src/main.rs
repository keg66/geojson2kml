use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::BTreeSet;
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

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
struct TrainLine<'a> {
    company_name: &'a str,
    line_name: &'a str,
}

fn get_string_from_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn search_candidates<'a>(query: &str, geo: &'a Geo) -> BTreeSet<TrainLine<'a>> {
    let mut candidates = BTreeSet::new();

    for feature in &geo.features {
        if feature.properties.N02_003.contains(query) || feature.properties.N02_004.contains(query)
        {
            let candidate = TrainLine {
                company_name: &feature.properties.N02_004,
                line_name: &feature.properties.N02_003,
            };
            candidates.insert(candidate);
        }
    }

    candidates
}

fn choose_id<'a>(query_line_name: &str, candidates: &'a Vec<TrainLine<'a>>) -> BTreeSet<usize> {
    let candidate_num = candidates.len();
    if candidate_num == 1 {
        let mut ids = BTreeSet::new();
        ids.insert(0);
        return ids;
    }
    if candidate_num > 1 {
        let mut ids = BTreeSet::new();
        println!("candidates:");
        for id in 0..candidate_num {
            println!(
                "[{}] {} {}",
                id, &candidates[id].company_name, &candidates[id].line_name
            );
        }

        loop {
            println!("choose '{}'...'{}' or : 'q' to exit", 0, candidate_num - 1);
            let answers = get_string_from_stdin();
            if answers == "q" {
                return BTreeSet::new();
            }

            let answers: Vec<&str> = answers.split(",").collect();
            for answer in answers {
                match answer.parse::<usize>() {
                    Ok(chosen_id) => {
                        if chosen_id >= candidate_num {
                            eprintln!("{} is invalid...", answer);
                            ids.clear();
                            break;
                        }
                        ids.insert(chosen_id);
                    }
                    Err(_) => {
                        eprintln!("{} is invalid...", answer);
                        ids.clear();
                        break;
                    }
                }
            }

            if !ids.is_empty() {
                break;
            }
        }
        return ids;
    }
    eprintln!("{} is not found...", query_line_name);
    BTreeSet::new()
}

fn generate_kml(train_lines: Vec<&TrainLine>, geo: &Geo) -> std::io::Result<()> {
    let mut filename = String::new();
    for train_line in &train_lines {
        let company_name = train_line.company_name;
        let line_name = train_line.line_name;
        filename = if filename.is_empty() {
            format!("{}-{}", company_name, line_name)
        } else {
            format!("{}_{}-{}", filename, company_name, line_name)
        };
    }
    let filename_with_ext = format!("{}.kml", filename);

    println!("creating {} ...", filename_with_ext);
    let mut file = File::create(&filename_with_ext)?;

    write!(
        file,
        r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>
<name>{}</name>"#,
        filename
    )?;

    for train_line in &train_lines {
        write!(file, "{}", generate_kml_body(&train_line, &geo))?;
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

fn generate_kml_body(train_line: &TrainLine, geo: &Geo) -> String {
    let mut body = String::new();

    let company_name = train_line.company_name;
    let line_name = train_line.line_name;

    let mut id = 0;

    for feature in &geo.features {
        if feature.properties.N02_003 == line_name && feature.properties.N02_004 == company_name {
            for line in &feature.geometry.coordinates {
                body = format!(
                    "{}
<Placemark>
  <name>{} {} {}</name>
  <LineString>
    <coordinates>",
                    body, company_name, line_name, id
                );

                for coordinate in line {
                    assert_eq!(coordinate.len(), 2);
                    body = format!(
                        "{}
      {},{},0",
                        body, coordinate[0], coordinate[1]
                    );
                }

                body = format!(
                    "{}
    </coordinates>
  </LineString>
</Placemark>",
                    body
                );
            }
            id += 1;
        }
    }
    body
}

fn main() {
    let content = std::fs::read_to_string("N02-20_RailroadSection.geojson").unwrap();
    let geo: Geo = serde_json::from_str(&content).unwrap();

    loop {
        println!("==================================");
        println!("enter train company name or line name or 'q' to exit:");
        let query = get_string_from_stdin();
        if query == "q" {
            break;
        }

        let query = &query as &str;
        let candidates: Vec<TrainLine> = search_candidates(query, &geo).into_iter().collect();

        let chosen_ids = choose_id(query, &candidates);

        let flag_merge = if chosen_ids.len() > 1 {
            println!("merge lines? [yes/no]:");
            let answer = get_string_from_stdin();
            if answer == "yes" {
                true
            } else {
                false
            }
        } else {
            false
        };

        if flag_merge {
            let mut train_lines = Vec::new();
            for id in chosen_ids {
                train_lines.push(&candidates[id])
            }
            if let Err(_) = generate_kml(train_lines, &geo) {
                eprintln!("failed to generate kml ...");
            }
        } else {
            for id in chosen_ids {
                if let Err(_) = generate_kml(vec![&candidates[id]], &geo) {
                    eprintln!("failed to generate kml ...");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_kml() {
        let geo_content = r#"{
            "type": "FeatureCollection",
            "name": "N02-20_RailroadSection",
            "crs": {
                "type": "name",
                "properties": {
                    "name": "urn:ogc:def:crs:EPSG::4326"
                }
            },
            "features": [
                {
                    "type": "Feature",
                    "properties": {
                        "N02_001": "1",
                        "N02_002": "2",
                        "N02_003": "line1",
                        "N02_004": "company1"
                    },
                    "geometry": {
                        "type": "MultiLineString",
                        "coordinates": [
                            [
                                [139.767, 35.681],
                                [139.768, 35.682]
                            ],
                            [
                                [139.769, 35.683],
                                [139.77, 35.684]
                            ]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": {
                        "N02_001": "3",
                        "N02_002": "4",
                        "N02_003": "line2",
                        "N02_004": "company2"
                    },
                    "geometry": {
                        "type": "MultiLineString",
                        "coordinates": [
                            [
                                [139.771, 35.685],
                                [139.772, 35.686]
                            ]
                        ]
                    }
                }
            ]
        }"#;
        let geo: Geo = serde_json::from_str(geo_content).unwrap();

        let train_line1 = TrainLine {
            company_name: "company1",
            line_name: "line1",
        };
        let kml_body = generate_kml_body(&train_line1, &geo);
        let expected_kml = r#"
<Placemark>
  <name>company1 line1 0</name>
  <LineString>
    <coordinates>
      139.767,35.681,0
      139.768,35.682,0
    </coordinates>
  </LineString>
</Placemark>
<Placemark>
  <name>company1 line1 0</name>
  <LineString>
    <coordinates>
      139.769,35.683,0
      139.77,35.684,0
    </coordinates>
  </LineString>
</Placemark>"#;
        assert_eq!(
            kml_body.trim().replace("\r\n", "\n"),
            expected_kml.trim().replace("\r\n", "\n"),
            "Generated KML does not match the expected KML"
        );
    }
}
