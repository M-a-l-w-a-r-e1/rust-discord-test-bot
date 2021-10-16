use reqwest;
fn main() {
    let client = reqwest::Client::new();
    let json = r#"
    {
        "content": null,
        "embeds": [
          {
            "title": "Libtards",
            "color": null
          }
        ]
      }
    "#;
    let v: serde_json::Value = serde_json::from_str(&json).expect("Unable to parse");

    let res = client.post("WEBHOOK_TOKEN").json(&v).send();
    println!("{:#?}",res);
}
