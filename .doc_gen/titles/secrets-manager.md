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
  secrets-manager: {CreateSecret: '', DescribeSecret: '', GetSecretValue: '',
  PutSecretValue: '', UpdateSecret: '', DeleteSecret: '', ListSecrets: '',
  BatchGetSecretValue: ''}
---
- [Creating Secrets in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_CreateSecret.html)
- [Describing Secrets in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_DescribeSecret.html)
- [Retrieving Secret Values in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html)
- [Storing Secret Values in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_PutSecretValue.html)
- [Updating Secrets in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_UpdateSecret.html)
- [Deleting Secrets in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_DeleteSecret.html)
- [Listing Secrets in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_ListSecrets.html)
- [Batch Retrieving Secret Values in AWS Secrets Manager](https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_BatchGetSecretValue.html)