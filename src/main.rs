use geojson2kml::{Geo, TrainLine, search_candidates, generate_kml_body, generate_filename};
use serde_json;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;

fn get_string_from_stdin() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
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
    let filename = generate_filename(&train_lines);
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
