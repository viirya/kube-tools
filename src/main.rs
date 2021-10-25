use anyhow;
use clap::{App, AppSettings, Arg};
use kube::core::object::ObjectList;
use kube::{
    api::{Api, DynamicObject},
    discovery, Client, ResourceExt,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Kube tools")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Utility to help apply Kube YAML")
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
    let api_services = get_api_services(client.clone(), "ais-sre.apple.com").await?;
    let configmaps = get_configmaps(client.clone(), "").await?;

    /*
    let narrarive_issuer = if api_services.len() > 0 {
        api_services.get(0);
    } else {
        kube::Error::
    }
     */

    Ok(())
}

async fn get_k8s_resources(
    client: Client,
    apigroup: &str,
    kind: &str,
) -> Result<ObjectList<DynamicObject>, kube::Error> {
    let apigroup = discovery::group(&client, apigroup).await?;
    let (ar, caps) = apigroup.recommended_kind(kind).unwrap();
    let api: Api<DynamicObject> = Api::all_with(client.clone(), &ar);
    let services = api.list(&Default::default()).await?;

    Ok(services)
}

async fn get_api_services(
    client: Client,
    apigroup: &str,
) -> Result<Vec<DynamicObject>, kube::Error> {
    let services = get_k8s_resources(client, apigroup, "NarrativeIssuer").await?;

    for service in &services {
        println!("Found APIService: {}", service.name());
    }

    Ok(services.items)
}

async fn get_configmaps(client: Client, apigroup: &str) -> Result<Vec<DynamicObject>, kube::Error> {
    let services = get_k8s_resources(client, apigroup, "ConfigMap").await?;

    for service in &services {
        println!("ConfigMap: {}", service.name());
    }

    Ok(services.items)
}
