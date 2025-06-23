use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub mod error;
pub mod io;
pub mod ui;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geo {
    pub r#type: String,
    pub name: String,
    pub crs: Crs,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crs {
    pub r#type: String,
    pub properties: Property,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Property {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub r#type: String,
    pub properties: GeoProperty,
    pub geometry: Geometry,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeoProperty {
    pub N02_001: String,
    pub N02_002: String,
    pub N02_003: String,
    pub N02_004: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<Vec<Vec<f32>>>,
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct TrainLine<'a> {
    pub company_name: &'a str,
    pub line_name: &'a str,
}

pub fn search_candidates<'a>(query: &str, geo: &'a Geo) -> BTreeSet<TrainLine<'a>> {
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

pub fn generate_kml_body(train_line: &TrainLine, geo: &Geo) -> String {
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

pub fn generate_filename(train_lines: &[&TrainLine]) -> String {
    let mut filename = String::new();
    for train_line in train_lines {
        let company_name = train_line.company_name;
        let line_name = train_line.line_name;
        filename = if filename.is_empty() {
            format!("{}-{}", company_name, line_name)
        } else {
            format!("{}_{}-{}", filename, company_name, line_name)
        };
    }
    filename
}