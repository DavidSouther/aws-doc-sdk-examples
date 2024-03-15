---
combined: true
debug:
  engine: bedrock
  finish: end_turn
  id: null
  model: anthropic.claude-3-sonnet-20240229-v1:0
  usage: null
isolated: true
prompt: '"iot-data-plane: {GetThingShadow: '''', UpdateThingShadow: ''''} "'
---
1. &iot-data-plane; - [Getting Thing Shadow State](&GetThingShadow;https://docs.aws.amazon.com/iot-data/latest/apireference/API_GetThingShadow.html)
2. &iot-data-plane; - [Updating Thing Shadow State](&UpdateThingShadow;https://docs.aws.amazon.com/iot-data/latest/apireference/API_UpdateThingShadow.html)