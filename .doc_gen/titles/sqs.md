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
  sqs: {ListQueues: '', CreateQueue: '', GetQueueUrl: '', DeleteQueue: '',
  SendMessage: '', SendMessageBatch: '', ReceiveMessage: '', DeleteMessage: '',
  DeleteMessageBatch: '', GetQueueAttributes: '', ChangeMessageVisibility: '',
  SetQueueAttributes: ''}
---
- [Listing Queues in Amazon SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_ListQueues.html)
- [Creating a Queue in Amazon SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_CreateQueue.html)
- [Getting a Queue URL in Amazon SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_GetQueueUrl.html)
- [Deleting a Queue in Amazon SQS](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_DeleteQueue.html)
- [Sending a Message to an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessage.html)
- [Sending a Batch of Messages to an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SendMessageBatch.html)
- [Receiving Messages from an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_ReceiveMessage.html)
- [Deleting a Message from an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_DeleteMessage.html)
- [Deleting a Batch of Messages from an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_DeleteMessageBatch.html)
- [Getting Attributes of an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_GetQueueAttributes.html)
- [Changing Message Visibility in an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_ChangeMessageVisibility.html)
- [Setting Attributes of an Amazon SQS Queue](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/APIReference/API_SetQueueAttributes.html)