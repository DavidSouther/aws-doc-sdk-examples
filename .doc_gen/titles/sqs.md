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
  "sqs: {ListQueues: '', CreateQueue: '', GetQueueUrl: '', DeleteQueue: '',
  SendMessage: '', SendMessageBatch: '', ReceiveMessage: '', DeleteMessage: '',
  DeleteMessageBatch: '', GetQueueAttributes: '', ChangeMessageVisibility: '',
  SetQueueAttributes: ''} Entities: SQSlong SQS"
---
&SQSlong;
- [Listing Queues in &SQS;](https://docs.aws.amazon.com/sqs/latest/APIReference/API_ListQueues.html)
- [Creating a Queue in &SQS;](https://docs.aws.amazon.com/sqs/latest/APIReference/API_CreateQueue.html)
- [Getting Queue URL in &SQS;](https://docs.aws.amazon.com/sqs/latest/APIReference/API_GetQueueUrl.html)
- [Deleting a Queue in &SQS;](https://docs.aws.amazon.com/sqs/latest/APIReference/API_DeleteQueue.html)
- [Sending a Message to a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_SendMessage.html)
- [Sending a Batch of Messages to a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_SendMessageBatch.html)
- [Receiving Messages from a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_ReceiveMessage.html)
- [Deleting a Message from a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_DeleteMessage.html)
- [Deleting a Batch of Messages from a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_DeleteMessageBatch.html)
- [Getting &SQS; Queue Attributes](https://docs.aws.amazon.com/sqs/latest/APIReference/API_GetQueueAttributes.html)
- [Changing Message Visibility in a &SQS; Queue](https://docs.aws.amazon.com/sqs/latest/APIReference/API_ChangeMessageVisibility.html)
- [Setting &SQS; Queue Attributes](https://docs.aws.amazon.com/sqs/latest/APIReference/API_SetQueueAttributes.html)