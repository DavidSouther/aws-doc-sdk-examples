---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: 'iot-data-plane: {GetThingShadow: '''', UpdateThingShadow: ''''}'
---
- [Retrieving Device State with AWS IoT Core Data Plane](https://docs.aws.amazon.com/iot-data/latest/apireference/API_GetThingShadow.html)
- [Updating Device State with AWS IoT Core Data Plane](https://docs.aws.amazon.com/iot-data/latest/apireference/API_UpdateThingShadow.html)