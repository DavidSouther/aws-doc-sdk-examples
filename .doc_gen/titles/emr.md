---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: >-
  emr: {RunJobFlow: '', DescribeCluster: '', TerminateJobFlows: '',
  AddJobFlowSteps: '', ListSteps: '', DescribeStep: '', ListInstances: ''}
---
- [Initiating a Cluster in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_RunJobFlow.html)
- [Viewing Cluster Details in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_DescribeCluster.html)
- [Terminating Clusters in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_TerminateJobFlows.html)
- [Adding Steps to a Cluster in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_AddJobFlowSteps.html)
- [Listing Steps for a Cluster in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_ListSteps.html)
- [Viewing Step Details in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_DescribeStep.html)
- [Listing Instances in a Cluster in Amazon EMR](https://docs.aws.amazon.com/emr/latest/APIReference/API_ListInstances.html)