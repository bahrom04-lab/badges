use std::collections::HashMap;

pub async fn generate_badge(instance: String, owner: String, repo: String) -> String {
    let link = format!("https://{instance}/api/v1/repos/{owner}/{repo}/languages");
    let mut body = reqwest::get(link)
        .await
        .expect("Couldn't send GET request")
        .text()
        .await
        .expect("Couldn't read text");

    body = body
        .replace("\"", "")
        .replace("\n", "")
        .replace("{", "")
        .replace("}", "");

    let parts = body.split(",").collect::<Vec<&str>>();
    // println!("Body: {parts:#?}");

    let mut table: HashMap<String, u64> = HashMap::new();
    for part in parts.clone() {
        let part = part.split(":").collect::<Vec<&str>>();
        let lang = part
            .first()
            .expect("Couldn't extract the language")
            .to_string();
        let bytes = part
            .get(1)
            .expect("Couldn't extract the bytes")
            .to_string()
            .parse::<u64>()
            .expect("Couldn't parse u64");

        table.insert(lang, bytes);
    }
    // println!("Table: {table:#?}");

    let mut total = 0;
    for (_, value) in table.iter() {
        total += value;
    }

    let mut entries = table.clone().into_iter().collect::<Vec<(String, u64)>>();
    entries.sort_by(|el1, el2| el2.1.cmp(&el1.1));
    let entry = entries.first().unwrap().to_owned();
    let lang = entry.0;
    let bytes = entry.1;

    let percentage = ((bytes as f64 / total as f64) * 100f64)
        // let percentage = ((420308f64 / total as f64) * 100f64)
        .to_string()
        .split(".")
        .next()
        .expect("Couldn't get the percentage")
        .parse::<u64>()
        .expect("Couldn't parse the percentage");

    // println!(
    //     "Language: {}, Bytes: {}, Total: {}, Percentage: {}",
    //     lang, bytes, total, percentage
    // );

    let link = format!("https://img.shields.io/badge/{lang}-{percentage}%25-blue?logo={lang}");
    // println!("Link: {link}");

    link
}
