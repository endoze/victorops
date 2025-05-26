use std::env;

#[tokio::main]
async fn main() -> victorops::ApiResult<()> {
  let api_id =
    env::var("VICTOROPS_API_ID").expect("VICTOROPS_API_ID environment variable must be set");
  let api_key =
    env::var("VICTOROPS_API_KEY").expect("VICTOROPS_API_KEY environment variable must be set");
  let base_url =
    env::var("VICTOROPS_BASE_URL").unwrap_or_else(|_| "https://api.victorops.com".to_string());

  let client = victorops::Client::new(api_id, api_key, base_url)?;

  if let Some(incident_id_str) = env::args().nth(1) {
    if let Ok(incident_id) = incident_id_str.parse::<i32>() {
      let (incident, _details) = client.get_incident(incident_id).await?;
      println!("Incident {}:", incident_id);
      println!("{:#?}", incident);
    } else {
      eprintln!("Invalid incident ID: {}", incident_id_str);
      std::process::exit(1);
    }
  } else {
    let (incidents, _details) = client.get_incidents().await?;
    println!("All Incidents:");
    println!("{:#?}", incidents);
  }

  Ok(())
}
