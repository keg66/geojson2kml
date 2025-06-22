use geojson2kml::{Geo, Feature, GeoProperty, Geometry, Crs, Property, TrainLine, search_candidates, generate_kml_body, generate_filename};

fn create_test_geo() -> Geo {
    Geo {
        r#type: "FeatureCollection".to_string(),
        name: "test".to_string(),
        crs: Crs {
            r#type: "name".to_string(),
            properties: Property {
                name: "test-crs".to_string(),
            },
        },
        features: vec![
            // テスト線A - ABC鉄道 (単一セグメント)
            Feature {
                r#type: "Feature".to_string(),
                properties: GeoProperty {
                    N02_001: "01".to_string(),
                    N02_002: "1".to_string(),
                    N02_003: "テスト線A".to_string(),
                    N02_004: "ABC鉄道".to_string(),
                },
                geometry: Geometry {
                    r#type: "MultiLineString".to_string(),
                    coordinates: vec![vec![
                        vec![100.0, 200.0],
                        vec![101.0, 201.0],
                    ]],
                },
            },
            // テスト線A - ABC鉄道 (追加セグメント)
            Feature {
                r#type: "Feature".to_string(),
                properties: GeoProperty {
                    N02_001: "01".to_string(),
                    N02_002: "1".to_string(),
                    N02_003: "テスト線A".to_string(),
                    N02_004: "ABC鉄道".to_string(),
                },
                geometry: Geometry {
                    r#type: "MultiLineString".to_string(),
                    coordinates: vec![vec![
                        vec![102.0, 202.0],
                        vec![103.0, 203.0],
                        vec![104.0, 204.0],
                    ]],
                },
            },
            // テスト線B - XYZ鉄道 (単一セグメント)
            Feature {
                r#type: "Feature".to_string(),
                properties: GeoProperty {
                    N02_001: "02".to_string(),
                    N02_002: "2".to_string(),
                    N02_003: "テスト線B".to_string(),
                    N02_004: "XYZ鉄道".to_string(),
                },
                geometry: Geometry {
                    r#type: "MultiLineString".to_string(),
                    coordinates: vec![vec![
                        vec![300.0, 400.0],
                        vec![301.0, 401.0],
                    ]],
                },
            },
            // 山手線 - JR東日本 (検索テスト用)
            Feature {
                r#type: "Feature".to_string(),
                properties: GeoProperty {
                    N02_001: "03".to_string(),
                    N02_002: "3".to_string(),
                    N02_003: "山手線".to_string(),
                    N02_004: "JR東日本".to_string(),
                },
                geometry: Geometry {
                    r#type: "MultiLineString".to_string(),
                    coordinates: vec![vec![
                        vec![500.0, 600.0],
                        vec![501.0, 601.0],
                    ]],
                },
            },
        ],
    }
}

#[test]
fn test_search_candidates_exact_line_name() {
    let geo = create_test_geo();
    let candidates = search_candidates("テスト線A", &geo);
    
    assert_eq!(candidates.len(), 1);
    let train_line = candidates.first().unwrap();
    assert_eq!(train_line.company_name, "ABC鉄道");
    assert_eq!(train_line.line_name, "テスト線A");
}

#[test]
fn test_search_candidates_partial_line_name() {
    let geo = create_test_geo();
    let candidates = search_candidates("山手", &geo);
    
    assert_eq!(candidates.len(), 1);
    let train_line = candidates.first().unwrap();
    assert_eq!(train_line.company_name, "JR東日本");
    assert_eq!(train_line.line_name, "山手線");
}

#[test]
fn test_search_candidates_company_name() {
    let geo = create_test_geo();
    let candidates = search_candidates("ABC", &geo);
    
    assert_eq!(candidates.len(), 1);
    let train_line = candidates.first().unwrap();
    assert_eq!(train_line.company_name, "ABC鉄道");
    assert_eq!(train_line.line_name, "テスト線A");
}

#[test]
fn test_search_candidates_no_match() {
    let geo = create_test_geo();
    let candidates = search_candidates("存在しない線", &geo);
    
    assert_eq!(candidates.len(), 0);
}

#[test]
fn test_search_candidates_multiple_results() {
    let geo = create_test_geo();
    let candidates = search_candidates("テスト", &geo);
    
    assert_eq!(candidates.len(), 2);
    let lines: Vec<&str> = candidates.iter().map(|t| t.line_name).collect();
    assert!(lines.contains(&"テスト線A"));
    assert!(lines.contains(&"テスト線B"));
}

#[test]
fn test_generate_kml_body_single_segment() {
    let geo = create_test_geo();
    let train_line = TrainLine {
        company_name: "XYZ鉄道",
        line_name: "テスト線B",
    };
    
    let kml_body = generate_kml_body(&train_line, &geo);
    
    // 基本構造チェック
    assert!(kml_body.contains("<Placemark>"));
    assert!(kml_body.contains("</Placemark>"));
    assert!(kml_body.contains("<LineString>"));
    assert!(kml_body.contains("<coordinates>"));
    assert!(kml_body.contains("XYZ鉄道 テスト線B 0"));
    
    // 座標チェック
    assert!(kml_body.contains("300,400,0"));
    assert!(kml_body.contains("301,401,0"));
    
    // Placemark数チェック
    let placemark_count = kml_body.matches("<Placemark>").count();
    assert_eq!(placemark_count, 1);
}

#[test]
fn test_generate_kml_body_multiple_segments() {
    let geo = create_test_geo();
    let train_line = TrainLine {
        company_name: "ABC鉄道",
        line_name: "テスト線A",
    };
    
    let kml_body = generate_kml_body(&train_line, &geo);
    
    // 複数セグメントのチェック
    let placemark_count = kml_body.matches("<Placemark>").count();
    assert_eq!(placemark_count, 2);
    
    // 各セグメントの名前チェック
    assert!(kml_body.contains("ABC鉄道 テスト線A 0"));
    assert!(kml_body.contains("ABC鉄道 テスト線A 1"));
    
    // 座標チェック
    assert!(kml_body.contains("100,200,0"));
    assert!(kml_body.contains("101,201,0"));
    assert!(kml_body.contains("102,202,0"));
    assert!(kml_body.contains("103,203,0"));
    assert!(kml_body.contains("104,204,0"));
}

#[test]
fn test_generate_kml_body_no_match() {
    let geo = create_test_geo();
    let train_line = TrainLine {
        company_name: "存在しない鉄道",
        line_name: "存在しない線",
    };
    
    let kml_body = generate_kml_body(&train_line, &geo);
    
    // 空の結果
    assert_eq!(kml_body.trim(), "");
}

#[test]
fn test_generate_filename_single_line() {
    let train_line = TrainLine {
        company_name: "ABC鉄道",
        line_name: "テスト線A",
    };
    
    let filename = generate_filename(&[&train_line]);
    assert_eq!(filename, "ABC鉄道-テスト線A");
}

#[test]
fn test_generate_filename_multiple_lines() {
    let train_line1 = TrainLine {
        company_name: "ABC鉄道",
        line_name: "テスト線A",
    };
    let train_line2 = TrainLine {
        company_name: "XYZ鉄道",
        line_name: "テスト線B",
    };
    
    let filename = generate_filename(&[&train_line1, &train_line2]);
    assert_eq!(filename, "ABC鉄道-テスト線A_XYZ鉄道-テスト線B");
}

#[test]
fn test_generate_filename_empty_lines() {
    let filename = generate_filename(&[]);
    assert_eq!(filename, "");
}

#[test]
fn test_trainline_ordering() {
    let line1 = TrainLine {
        company_name: "ABC鉄道",
        line_name: "テスト線A",
    };
    let line2 = TrainLine {
        company_name: "XYZ鉄道",
        line_name: "テスト線B",
    };
    let line3 = TrainLine {
        company_name: "ABC鉄道",
        line_name: "テスト線B",
    };
    
    // BTreeSetでの順序確認
    use std::collections::BTreeSet;
    let mut set = BTreeSet::new();
    set.insert(line2.clone());
    set.insert(line1.clone());
    set.insert(line3.clone());
    
    let ordered: Vec<_> = set.into_iter().collect();
    assert_eq!(ordered[0], line1); // ABC鉄道-テスト線A
    assert_eq!(ordered[1], line3); // ABC鉄道-テスト線B  
    assert_eq!(ordered[2], line2); // XYZ鉄道-テスト線B
}