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
  dynamodb: {ListTables: '', CreateTable: '', BatchGetItem: '', DescribeTable:
  '', BatchWriteItem: '', DeleteTable: '', PutItem: '', GetItem: '', UpdateItem:
  '', DeleteItem: '', Query: '', Scan: '', ExecuteStatement: '',
  BatchExecuteStatement: ''}
---
- [List DynamoDB Tables](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ListTables.html)
- [Creating a DynamoDB Table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_CreateTable.html)
- [Batch Getting Items from DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchGetItem.html)
- [Describing a DynamoDB Table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DescribeTable.html)
- [Batch Writing Items to DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchWriteItem.html)
- [Deleting a DynamoDB Table](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteTable.html)
- [Putting an Item in DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_PutItem.html)
- [Getting an Item from DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_GetItem.html)
- [Updating an Item in DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_UpdateItem.html)
- [Deleting an Item from DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_DeleteItem.html)
- [Querying DynamoDB Tables](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Query.html)
- [Scanning DynamoDB Tables](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_Scan.html)
- [Executing a Statement in DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_ExecuteStatement.html)
- [Batch Executing Statements in DynamoDB](https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchExecuteStatement.html)