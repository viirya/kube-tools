use clap::{App, AppSettings, Arg};
use kube::{
    api::{Api, DynamicObject},
    discovery, Client, ResourceExt,
};

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let matches = App::new("Kube tools")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Utility to help apply Kube YAML")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::DisableVersionForSubcommands)
        .subcommand(
            App::new("apply")
                .setting(AppSettings::ArgRequiredElseHelp)
                .about("apply the given YAML file")
                .args(&[Arg::new("yaml").about("YAML file").required(true)]),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("apply", apply_matches)) => {
            let yaml_file = apply_matches.value_of("yaml").unwrap();
        }
        _ => unreachable!(),
    }

    let client = Client::try_default().await?;
    get_api_services(client.clone(), "ais-sre.apple.com");
    get_configmaps(client.clone(), "");

    Ok(())
}

async fn get_api_services(client: Client, apigroup: &str) -> Result<(), kube::Error> {
    let apigroup = discovery::group(&client, apigroup).await?;
    let (ar, caps) = apigroup.recommended_kind("NarrativeIssuer").unwrap();
    let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
    for service in api.list(&Default::default()).await? {
        println!("Found APIService: {}", service.name());
    }

    Ok(())
}

async fn get_configmaps(client: Client, apigroup: &str) -> Result<(), kube::Error> {
    let apigroup = discovery::group(&client, apigroup).await?;
    let (ar, caps) = apigroup.recommended_kind("ConfigMap").unwrap();
    let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
    for service in api.list(&Default::default()).await? {
        println!("ConfigMap: {}", service.name());
    }

    Ok(())
}
