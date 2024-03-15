---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: 'cloudwatch-events: {PutRule: '''', PutEvents: '''', PutTargets: ''''}'
---
1. cloudwatch-events: PutRule
   - [Defining Event Rules in Amazon CloudWatch Events](https://docs.aws.amazon.com/AmazonCloudWatchEvents/latest/APIReference/API_PutRule.html)

2. cloudwatch-events: PutEvents
   - [Triggering Event-Driven Workflows in Amazon CloudWatch Events](https://docs.aws.amazon.com/AmazonCloudWatchEvents/latest/APIReference/API_PutEvents.html)

3. cloudwatch-events: PutTargets
   - [Configuring Event Targets in Amazon CloudWatch Events](https://docs.aws.amazon.com/AmazonCloudWatchEvents/latest/APIReference/API_PutTargets.html)