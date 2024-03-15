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
  ecs: {ListClusters: '', CreateCluster: '', DeleteCluster: '',
  DescribeClusters: '', CreateService: '', DeleteService: '', DescribeTasks: '',
  ListServices: '', ListTasks: '', UpdateService: ''}
---
- [Listing Amazon ECS Clusters](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListClusters.html)
- [Creating an Amazon ECS Cluster](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateCluster.html)
- [Deleting an Amazon ECS Cluster](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_DeleteCluster.html)
- [Describing Amazon ECS Clusters](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_DescribeClusters.html)
- [Creating an Amazon ECS Service](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_CreateService.html)
- [Deleting an Amazon ECS Service](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_DeleteService.html)
- [Describing Amazon ECS Tasks](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_DescribeTasks.html)
- [Listing Amazon ECS Services](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListServices.html)
- [Listing Amazon ECS Tasks](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_ListTasks.html)
- [Updating an Amazon ECS Service](https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_UpdateService.html)