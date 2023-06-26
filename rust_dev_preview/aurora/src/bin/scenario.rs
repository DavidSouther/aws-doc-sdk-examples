/*

## Scenario

A scenario runs at a command prompt and prints output to the user on the result of each service action. A scenario can run in one of two ways: straight through, printing out progress as it goes, or as an interactive question/answer script.

### Getting started with Aurora Clusters

1. Get available engine families for Aurora MySql. rds.DescribeDbEngineVersions(Engine='aurora-mysql') and build a set of the 'DBParameterGroupFamily' field values. I get {aurora-mysql8.0, aurora-mysql5.7}.
2. Select an engine family and create a custom DB cluster parameter group. rds.CreateDbClusterParameterGroup(DBParameterGroupFamily='aurora-mysql8.0')
3. Get the parameter group. rds.DescribeDbClusterParameterGroups
4. Get parameters in the group. This is a long list so you will have to paginate. Find the auto_increment_offset and auto_increment_increment parameters (by ParameterName). rds.DescribeDbClusterParameters
5. Parse the ParameterName, Description, and AllowedValues values and display them.
6. Modify both the auto_increment_offset and auto_increment_increment parameters in one call in the custom parameter group. Set their ParameterValue fields to a new allowable value. rds.ModifyDbClusterParameterGroup.
7. Get and display the updated parameters. Specify Source of 'user' to get just the modified parameters. rds.DescribeDbClusterParameters(Source='user')
8. Get a list of allowed engine versions. rds.DescribeDbEngineVersions(Engine='aurora-mysql', DBParameterGroupFamily=<the family used to create your parameter group in step 2>)
9. Create an Aurora DB cluster database cluster that contains a MySql database and uses the parameter group you created.
```
self.rds_client.create_db_cluster(
                DatabaseName=db_name,
                DBClusterIdentifier=cluster_name,
                DBClusterParameterGroupName=parameter_group_name,
                Engine=db_engine,
                EngineVersion=db_engine_version,
                MasterUsername=admin_name,
                MasterUserPassword=admin_password)
```
10. Wait for DB cluster to be ready. Call rds.DescribeDBClusters and check for Status == 'available'.
11. Get a list of instance classes available for the selected engine and engine version. rds.DescribeOrderableDbInstanceOptions(Engine='mysql', EngineVersion=).
12. Create a database instance in the cluster.
```
self.rds_client.create_db_instance(
                DBInstanceIdentifier=instance_id,
                DBClusterIdentifier=cluster_id,
                Engine=db_engine,
                DBInstanceClass=instance_class)
```
13. Wait for DB instance to be ready. Call rds.DescribeDbInstances and check for DBInstanceStatus == 'available'.
14. Display the connection string that can be used to connect a 'mysql' shell to the cluster. In Python:
```
    print(f"\n\tmysql -h {cluster['Endpoint']} -P {cluster['Port']} -u {cluster['MasterUsername']} -p\n")
```
15. Create a snapshot of the DB cluster.  rds.CreateDbClusterSnapshot.
16. Wait for the snapshot to create. rds.DescribeDbClusterSnapshots until Status == 'available'.
17. Delete the instance. rds.DeleteDbInstance.
18. Delete the DB cluster. rds.DeleteDbCluster.
19. Wait for the instance and cluster to fully delete. rds.DescribeDbInstances and rds.DescribeDbClusters until both are not found.
20. Delete the DB cluster parameter group. rds.DeleteDbClusterParameterGroup.

## Metadata

In `aurora_metadata`:

aurora_DescribeDbClusterParameterGroups
aurora_CreateDbClusterParameterGroup
aurora_ModifyDbClusterParameterGroup
aurora_DeleteDbClusterParameterGroup
aurora_DescribeDbEngineVersions
aurora_DescribeDbClusterParameters
aurora_DescribeDbClusters
aurora_CreateDbCluster
aurora_DeleteDbCluster
aurora_DescribeOrderableDbInstanceOptions
aurora_DescribeDbInstances
aurora_CreateDbInstance
aurora_DeleteDbInstance
aurora_CreateDbClusterSnapshot
aurora_DescribeDbClusterSnapshots
aurora_Scenario_GetStartedAuroraClusters
 */

use aws_sdk_rds::{
    operation::describe_db_cluster_parameter_groups::DescribeDbClusterParameterGroupsOutput, Client,
};
use cursive::views::{Dialog, TextView};
use std::{sync::Arc, thread};

struct AuroraScenario {}

struct AuroraCursiveUi {
    siv: &mut cursive::Cursive,
}

trait AuroraUi {
    pub fn error(&self, err: anyhow::Error);
    pub fn welcome(&self);
    // 1. Get available engine families for Aurora MySql. rds.DescribeDbEngineVersions(Engine='aurora-mysql') and build a set of the 'DBParameterGroupFamily' field values. I get {aurora-mysql8.0, aurora-mysql5.7}.
    // 2. Select an engine family and create a custom DB cluster parameter group. rds.CreateDbClusterParameterGroup(DBParameterGroupFamily='aurora-mysql8.0')
    pub fn select_engine_family(&self);
    // 3. Get the parameter group. rds.DescribeDbClusterParameterGroups
    // 4. Get parameters in the group. This is a long list so you will have to paginate. Find the auto_increment_offset and auto_increment_increment parameters (by ParameterName). rds.DescribeDbClusterParameters
    pub fn set_parameters(&self);
    // 5. Parse the ParameterName, Description, and AllowedValues values and display them.
    // 6. Modify both the auto_increment_offset and auto_increment_increment parameters in one call in the custom parameter group. Set their ParameterValue fields to a new allowable value. rds.ModifyDbClusterParameterGroup.
    // 7. Get and display the updated parameters. Specify Source of 'user' to get just the modified parameters. rds.DescribeDbClusterParameters(Source='user')
    // 8. Get a list of allowed engine versions. rds.DescribeDbEngineVersions(Engine='aurora-mysql', DBParameterGroupFamily=<the family used to create your parameter group in step 2>)
    // 9. Create an Aurora DB cluster database cluster that contains a MySql database and uses the parameter group you created.
    // 10. Wait for DB cluster to be ready. Call rds.DescribeDBClusters and check for Status == 'available'.
    pub fn show_clusters(&self);
    // 11. Get a list of instance classes available for the selected engine and engine version.
    // 12. Create a database instance in the cluster.
    // 13. Wait for DB instance to be ready. Call rds.DescribeDbInstances and check for DBInstanceStatus == 'available'.
    // 14. Display the connection string that can be used to connect a 'mysql' shell to the cluster. In Python:
    pub fn show_instance(&self);
    // 15. Create a snapshot of the DB cluster.  rds.CreateDbClusterSnapshot.
    // 16. Wait for the snapshot to create.
    pub fn create_snapshot(&self);
    // 17. Delete the instance. rds.DeleteDbInstance.
    // 18. Delete the DB cluster. rds.DeleteDbCluster.
    // 19. Wait for the instance and cluster to fully delete. rds.DescribeDbInstances and rds.DescribeDbClusters until both are not found.
    // 20. Delete the DB cluster parameter group. rds.DeleteDbClusterParameterGroup.
    pub fn cleanup(&self);
}

impl AuroraUi for AuroraCursiveUi {
    pub fn error(&self, err: anyhow::Error) {
        todo!()
    }

    pub fn welcome(&self) {
        todo!()
    }

    pub fn select_engine_family(&self) {
        todo!()
    }

    pub fn set_parameters(&self) {
        todo!()
    }

    pub fn show_clusters(&self) {
        todo!()
    }

    pub fn show_instance(&self) {
        todo!()
    }

    pub fn create_snapshot(&self) {
        todo!()
    }

    pub fn cleanup(&self) {
        todo!()
    }
}

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    let cb_sink = siv.cb_sink().clone();

    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let sdk_config = aws_config::load_from_env().await;
                let aurora_client = Client::new(&sdk_config);
                let db_cluster_parameter_groups = aurora_client
                    .describe_db_cluster_parameter_groups()
                    .send()
                    .await
                    .unwrap();
                cb_sink.send(Box::new(move |s| {
                    s.pop_layer();
                    s.add_layer(
                        Dialog::around(TextView::new(format!(
                            "Got Parameter Groups: {:?}",
                            db_cluster_parameter_groups
                        )))
                        .button("Quit", |s| s.quit()),
                    )
                }))
            })
            .unwrap();
    });

    // Creates a dialog with a single "Quit" button
    siv.add_layer(Dialog::around(TextView::new("Waiting for SDK...")).button("Quit", |s| s.quit()));

    // Starts the event loop.
    siv.run();
}
