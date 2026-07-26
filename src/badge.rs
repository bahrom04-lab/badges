use std::collections::HashMap;

pub async fn generate_badge(instance: String, owner: String, repo: String) -> String {
    let link = format!("https://{instance}/api/v1/repos/{owner}/{repo}/languages");
    let mut body = reqwest::get(link)
        .await
        .expect("Couldn't send GET request")
        .text()
        .await
        .expect("Couldn't read text");

    body.retain(|c| !matches!(c, '\n' | '"' | '{' | '}'));

    let parts = body.split(",");

    let table = parts
        .clone()
        .map(|p| p.split(":"))
        .map(|mut s| {
            (
                s.next().expect("Couldn't extract the language"),
                s.next()
                    .expect("Couldn't extract the bytes")
                    .parse::<u64>()
                    .expect("Couldn't parse u64"),
            )
        })
        .collect::<Vec<(&str, u64)>>()
        .into_iter()
        .collect::<HashMap<&str, u64>>();

    let mut total = 0;
    for (_, value) in table.iter() {
        total += value;
    }

    let mut entries = table.clone().into_iter().collect::<Vec<(&str, u64)>>();
    entries.sort_by(|el1, el2| el2.1.cmp(&el1.1));
    let entry = entries.first().unwrap().to_owned();
    let lang = entry.0;
    let bytes = entry.1;

    let percentage = ((bytes as f64 / total as f64) * 100f64)
        .to_string()
        .split(".")
        .next()
        .expect("Couldn't get the percentage")
        .parse::<u64>()
        .expect("Couldn't parse the percentage");

    let link = format!("https://img.shields.io/badge/{lang}-{percentage}%25-blue?logo={lang}");

    link
}
