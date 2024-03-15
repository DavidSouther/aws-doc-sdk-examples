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
  "config-service: {PutConfigRule: '', DescribeConfigRules: '',
  DeleteConfigRule: ''} "
---
- [Creating a &ConfigService; Rule](&UrlPutConfigRule;)
- [Listing &ConfigService; Rules](&UrlDescribeConfigRules;)  
- [Deleting a &ConfigService; Rule](&UrlDeleteConfigRule;)