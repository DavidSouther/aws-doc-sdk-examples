/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_rds::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Describe RDS Clusters
// snippet-start:[rds.rust.describe-clusters]
async fn describe_clusters(client: &Client) -> Result<(), Error> {
    let result = client.describe_db_clusters().send().await?;

    for db_cluster in result.db_clusters().unwrap_or_default() {
        println!("Name:  {}", db_cluster.database_name().unwrap_or("Unknown"));
        println!(
            "ID:    {}",
            db_cluster.db_cluster_identifier().unwrap_or("Unknown")
        );
        println!(
            "Zones: {:?}",
            db_cluster.availability_zones().unwrap_or_default()
        );
        println!();
    }

    Ok(())
}
// snippet-end:[rds.rust.describe-clusters]

/// Displays information about your Amazon Relational Database Service (Amazon RDS) clusters in the Region.
/// # Arguments
///
/// * `[-r REGION]` - The region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt { region, verbose } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));
    println!();

    if verbose {
        println!("RDS client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    describe_clusters(&client).await
}
