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

  let team_slug = env::args()
    .nth(1)
    .expect("Usage: cargo run --example team_schedule <team_slug>");

  let (schedule, _details) = client.get_api_team_schedule(&team_slug, 7, 0, 0).await?;

  if !schedule.schedules.is_empty() {
    println!("Team Schedule for '{}':", team_slug);
    println!("{:#?}", schedule.schedules);
  } else {
    println!("No schedules found for team '{}'", team_slug);
  }

  Ok(())
}
