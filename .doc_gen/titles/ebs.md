---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: 'ebs: {StartSnapshot: '''', PutSnapshotBlock: '''', CompleteSnapshot: ''''}'
---
ebs:

1. [Initiating an Amazon EBS Snapshot](https://docs.aws.amazon.com/ebs/latest/APIReference/API_StartSnapshot.html)
2. [Uploading Data to an Amazon EBS Snapshot](https://docs.aws.amazon.com/ebs/latest/APIReference/API_PutSnapshotBlock.html)
3. [Finalizing an Amazon EBS Snapshot](https://docs.aws.amazon.com/ebs/latest/APIReference/API_CompleteSnapshot.html)