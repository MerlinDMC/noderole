use clap::{crate_name, AppSettings, Parser};
use figment::{
    providers::{Format, Yaml},
    Figment,
};
use k8s_openapi::api::core::v1::Node;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{Api, Patch, PatchParams};
use kube::Client;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    /// Path to the config file
    #[clap(parse(from_os_str))]
    #[clap(short, long, default_value = "/etc/noderole.yml")]
    config: std::path::PathBuf,

    /// Kubernetes Node name
    #[clap(short, long)]
    #[clap(env = "NODE_NAME")]
    nodename: String,
}

#[derive(Deserialize)]
struct Config {
    /// Assignable node roles as a list of strings
    roles: Option<Vec<String>>,

    /// KV pairs of additional node labels to assign
    labels: Option<BTreeMap<String, String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config: Config = Figment::new().merge(Yaml::file(args.config)).extract()?;

    let client = Client::try_default().await?;
    let nodes: Api<Node> = Api::all(client.clone());

    // try reading the given node and hard fail if not available
    nodes.get(args.nodename.as_str()).await?;

    // map of labels to assign
    let mut node_labels: BTreeMap<String, String> = BTreeMap::new();

    // roles defined in the config file will be prefixed accordingly
    match config.roles {
        Some(roles) => {
            for role in roles {
                node_labels.insert(
                    format!("node-role.kubernetes.io/{}", role),
                    "true".to_string(),
                );
            }
        }
        None => {}
    }

    // additional raw KV labels will be appended as-is
    match config.labels {
        Some(labels) => node_labels.extend(labels),
        None => {}
    }

    let patch = Node {
        metadata: ObjectMeta {
            name: Some(args.nodename.clone()),
            labels: Some(node_labels),
            ..ObjectMeta::default()
        },
        spec: None,
        status: None,
    };

    let params = PatchParams::apply(crate_name!());
    let patch = Patch::Apply(&patch);

    nodes.patch(args.nodename.as_str(), &params, &patch).await?;

    Ok(())
}
