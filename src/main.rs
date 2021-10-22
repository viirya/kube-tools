use kube::{Client, api::{Api, DynamicObject}, discovery, ResourceExt};

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
  let client = Client::try_default().await?;
  let apigroup = discovery::group(&client, "ais-sre.apple.com").await?;
  let (ar, caps) = apigroup.recommended_kind("NarrativeIssuer").unwrap();
  let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
  for service in api.list(&Default::default()).await? {
      println!("Found APIService: {}", service.name());
  }
  Ok(())
}
