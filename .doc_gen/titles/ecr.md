---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: '"ecr: {DescribeRepositories: '''', ListImages: ''''} "'
---
- [Listing &ECR; Repositories](https://docs.aws.amazon.com/ecr/latest/APIReference/API_DescribeRepositories.html)
- [Viewing &ECR; Repository Images](https://docs.aws.amazon.com/ecr/latest/APIReference/API_ListImages.html)