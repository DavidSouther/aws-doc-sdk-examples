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
  "dynamodb: {ListTables: '', CreateTable: '', BatchGetItem: '', DescribeTable:
  '', BatchWriteItem: '', DeleteTable: '', PutItem: '', GetItem: '', UpdateItem:
  '', DeleteItem: '', Query: '', Scan: '', ExecuteStatement: '',
  BatchExecuteStatement: ''} Entities: DDBlong DDB"
---
- [&DDBlong; tables enumeration]( https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ListTables.html)
- [Creating &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_CreateTable.html)
- [Batch reading &DDBlong; items](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchGetItem.html)
- [Describing &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DescribeTable.html)
- [Batch writing &DDBlong; items](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchWriteItem.html)
- [Deleting &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteTable.html)
- [Adding item to &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html)
- [Retrieving &DDBlong; item](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html)
- [Updating &DDBlong; item](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html)
- [Deleting &DDBlong; item](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html)
- [Querying &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html)
- [Scanning &DDBlong; table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html)
- [Executing &DDBlong; statement](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ExecuteStatement.html)
- [Batch executing &DDBlong; statements](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchExecuteStatement.html)