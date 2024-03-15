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
  "elastic-load-balancing-v2: {DescribeLoadBalancers: '', CreateTargetGroup: '',
  DescribeTargetGroups: '', DeleteTargetGroup: '', CreateLoadBalancer: '',
  CreateListener: '', DeleteLoadBalancer: '', DescribeTargetHealth: ''} "
---
- [Creating a &ELBv2; Target Group](&describeloadbalancers;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeLoadBalancers.html)
- [Viewing &ELBv2; Target Groups](&createtargetgroup;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_CreateTargetGroup.html)
- [Managing &ELBv2; Target Groups](&describetargetgroups;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeTargetGroups.html)
- [Deleting an &ELBv2; Target Group](&deletetargetgroup;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DeleteTargetGroup.html)
- [Provisioning an &ELBv2; Load Balancer](&createloadbalancer;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_CreateLoadBalancer.html)
- [Configuring an &ELBv2; Listener](&createlistener;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_CreateListener.html)
- [Deprovisioning an &ELBv2; Load Balancer](&deleteloadbalancer;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DeleteLoadBalancer.html)
- [Monitoring &ELBv2; Target Health](&describetargethealth;https://docs.aws.amazon.com/elasticloadbalancing/latest/APIReference/API_DescribeTargetHealth.html)