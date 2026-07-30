pub async fn generate_badge(instance: &str, owner: &str, repo: &str) -> Box<str> {
    let url = format!("https://{instance}/api/v1/repos/{owner}/{repo}/languages");

    let Ok(res) = reqwest::get(url).await else {
        return "https://img.shields.io/404".into();
    };
    let Ok(table) = res.json::<std::collections::HashMap<Box<str>, u64>>().await else {
        return "https://img.shields.io/404".into();
    };

    let total: u64 = table.values().sum();

    let Some(entry) = table.into_iter().max_by(|(_, a), (_, b)| a.cmp(b)) else {
        return "https://img.shields.io/404".into();
    };
    let (lang, bytes) = entry;

    let percentage = ((bytes as f64 / total as f64) * 100f64).round() as u64;

    format!("https://img.shields.io/badge/{lang}-{percentage}%25-blue?logo={lang}").into()
}
