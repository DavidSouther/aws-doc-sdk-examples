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
  "ssm: {DescribeParameters: '', PutParameter: '', CreateOpsItem: '',
  DescribeOpsItems: '', UpdateOpsItem: ''} "
---
- [Describing &SSM; Parameters](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeParameters.html)
- [Putting a &SSM; Parameter](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_PutParameter.html)
- [Creating an &SSM; OpsItem](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_CreateOpsItem.html)
- [Describing &SSM; OpsItems](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_DescribeOpsItems.html)
- [Updating an &SSM; OpsItem](https://docs.aws.amazon.com/systems-manager/latest/APIReference/API_UpdateOpsItem.html)