/*/
## Service actions

Service actions can either be pulled out as individual functions or can be incorporated into the scenario, but each service action must be included as an excerpt in the SOS output.

RDS client:

DescribeDbClusterParameterGroups
CreateDbClusterParameterGroup
ModifyDbClusterParameterGroup
DeleteDbClusterParameterGroup
DescribeDbEngineVersions
DescribeDbClusterParameters
DescribeDbClusters
CreateDbCluster
DeleteDbCluster
DescribeOrderableDbInstanceOptions
DescribeDbInstances
CreateDbInstance
DeleteDbInstance
CreateDbClusterSnapshot
DescribeDbClusterSnapshots


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

use std::future::Future;
use std::pin::Pin;

pub struct GetEnginesResult;

pub trait AuroraActions {
    /// 1. Get available engine families for Aurora MySql. rds.DescribeDbEngineVersions(Engine='aurora-mysql') and build a set of the 'DBParameterGroupFamily' field values. I get {aurora-mysql8.0, aurora-mysql5.7}.
    fn get_engines(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<GetEnginesResult>> + '_>>;

    /// 2. Select an engine family and create a custom DB cluster parameter group. rds.CreateDbClusterParameterGroup(DBParameterGroupFamily='aurora-mysql8.0')
    /// 3. Get the parameter group. rds.DescribeDbClusterParameterGroups
    /// 4. Get parameters in the group. This is a long list so you will have to paginate. Find the auto_increment_offset and auto_increment_increment parameters (by ParameterName). rds.DescribeDbClusterParameters
    /// 5. Parse the ParameterName, Description, and AllowedValues values and display them.
    // pub sync fn prepare_groups(&self) -> Result<(), anyhow::Error>;

    /// 6. Modify both the auto_increment_offset and auto_increment_increment parameters in one call in the custom parameter group. Set their ParameterValue fields to a new allowable value. rds.ModifyDbClusterParameterGroup.
    /// 7. Get and display the updated parameters. Specify Source of 'user' to get just the modified parameters. rds.DescribeDbClusterParameters(Source='user')
    // pub async fn update_group(&self) -> Result<(), anyhow::Error>;

    /// 8. Get a list of allowed engine versions. rds.DescribeDbEngineVersions(Engine='aurora-mysql', DBParameterGroupFamily=<the family used to create your parameter group in step 2>)
    /// 9. Create an Aurora DB cluster database cluster that contains a MySql database and uses the parameter group you created.
    /// 10. Wait for DB cluster to be ready. Call rds.DescribeDBClusters and check for Status == 'available'.
    // pub async fn create_cluster(&self) -> Result<(), anyhow::Error>;

    /// 11. Get a list of instance classes available for the selected engine and engine version. rds.DescribeOrderableDbInstanceOptions(Engine='mysql', EngineVersion=).
    /// 12. Create a database instance in the cluster.
    /// 13. Wait for DB instance to be ready. Call rds.DescribeDbInstances and check for DBInstanceStatus == 'available'.
    /// 14. Display the connection string that can be used to connect a 'mysql' shell to the cluster.
    // pub async fn create_instance(&self) -> Result<(), anyhow::Error>;

    /// 15. Create a snapshot of the DB cluster.  rds.CreateDbClusterSnapshot.
    /// 16. Wait for the snapshot to create. rds.DescribeDbClusterSnapshots until Status == 'available'.
    // pub async fn create_snapshot(&self) -> Result<(), anyhow::Error>;

    /// 17. Delete the instance. rds.DeleteDbInstance.
    /// 18. Delete the DB cluster. rds.DeleteDbCluster.
    /// 19. Wait for the instance and cluster to fully delete. rds.DescribeDbInstances and rds.DescribeDbClusters until both are not found.
    /// 20. Delete the DB cluster parameter group. rds.DeleteDbClusterParameterGroup.
    fn cleanup(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + '_>>;
}

pub struct RdsClientAuroraActions {
    client: aws_sdk_rds::Client,
}

impl RdsClientAuroraActions {
    pub async fn load_from_env() -> Self {
        let config = aws_config::load_from_env().await;
        RdsClientAuroraActions {
            client: aws_sdk_rds::Client::new(&config),
        }
    }
}

impl AuroraActions for RdsClientAuroraActions {
    // DescribeDbEngineVersions
    fn get_engines(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<GetEnginesResult>> + '_>> {
        tokio::spawn()
        todo!()
    }

    fn cleanup(&self) -> Pin<Box<dyn Future<Output = anyhow::Result<()>> + '_>> {
        todo!()
    }
    // DescribeDbClusterParameterGroups
    // CreateDbClusterParameterGroup
    // ModifyDbClusterParameterGroup
    // DeleteDbClusterParameterGroup
    // DescribeDbClusterParameters
    // DescribeDbClusters
    // CreateDbCluster
    // DeleteDbCluster
    // DescribeOrderableDbInstanceOptions
    // DescribeDbInstances
    // CreateDbInstance
    // DeleteDbInstance
    // CreateDbClusterSnapshot
    // DescribeDbClusterSnapshots
}
