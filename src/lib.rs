use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use quick_xml::Writer;
use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use std::io::Cursor;

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
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut id = 0;

    let company_name = train_line.company_name;
    let line_name = train_line.line_name;

    for feature in &geo.features {
        if feature.properties.N02_003 == line_name && feature.properties.N02_004 == company_name {
            for line in &feature.geometry.coordinates {
                // <Placemark>
                writer.write_event(Event::Start(BytesStart::new("Placemark"))).unwrap();

                // <name>
                writer.write_event(Event::Start(BytesStart::new("name"))).unwrap();
                let name_text = format!("{} {} {}", company_name, line_name, id);
                writer.write_event(Event::Text(BytesText::new(&name_text))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("name"))).unwrap();

                // <LineString>
                writer.write_event(Event::Start(BytesStart::new("LineString"))).unwrap();

                // <coordinates>
                writer.write_event(Event::Start(BytesStart::new("coordinates"))).unwrap();
                
                let coordinates_text = line.iter()
                    .map(|coordinate| {
                        assert_eq!(coordinate.len(), 2);
                        format!("{},{},0", coordinate[0], coordinate[1])
                    })
                    .collect::<Vec<_>>()
                    .join("\n      ");
                let formatted_coords = format!("\n      {}\n    ", coordinates_text);
                
                writer.write_event(Event::Text(BytesText::new(&formatted_coords))).unwrap();
                writer.write_event(Event::End(BytesEnd::new("coordinates"))).unwrap();

                // </LineString>
                writer.write_event(Event::End(BytesEnd::new("LineString"))).unwrap();

                // </Placemark>
                writer.write_event(Event::End(BytesEnd::new("Placemark"))).unwrap();
            }
            id += 1;
        }
    }

    String::from_utf8(writer.into_inner().into_inner()).unwrap()
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