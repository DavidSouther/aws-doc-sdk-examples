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
  "secrets-manager: {CreateSecret: '', DescribeSecret: '', GetSecretValue: '',
  PutSecretValue: '', UpdateSecret: '', DeleteSecret: '', ListSecrets: '',
  BatchGetSecretValue: ''} "
---
- [Creating a &secrets-manager; secret](&secrets-manager-createSecret;)
- [Describing a &secrets-manager; secret](&secrets-manager-describesecret;)
- [Getting a &secrets-manager; secret value](&secrets-manager-getsecretvalue;)
- [Putting a &secrets-manager; secret value](&secrets-manager-putsecretvalue;)
- [Updating a &secrets-manager; secret](&secrets-manager-updatesecret;)
- [Deleting a &secrets-manager; secret](&secrets-manager-deletesecret;)
- [Listing &secrets-manager; secrets](&secrets-manager-listsecrets;)
- [Batch getting &secrets-manager; secret values](&secrets-manager-batchgetsecretvalue;)