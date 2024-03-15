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
  "medical-imaging: {ListDatastores: '', CreateDatastore: '', DeleteDatastore:
  '', GetDatastore: '', StartDICOMImportJob: '', GetDICOMImportJob: '',
  ListDICOMImportJobs: '', SearchImageSets: '', GetImageSet: '',
  GetImageSetMetadata: '', GetImageFrame: '', ListImageSetVersions: '',
  UpdateImageSetMetadata: '', CopyImageSet: '', DeleteImageSet: '', TagResource:
  '', UntagResource: '', ListTagsForResource: ''} Entities: AHIlong AHI"
---
- &AHIlong; managing datastores
    - [Listing &AHIlong; datastores](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_ListDatastores.html)
    - [Creating a &AHIlong; datastore](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_CreateDatastore.html)
    - [Deleting a &AHIlong; datastore](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_DeleteDatastore.html)
    - [Getting a &AHIlong; datastore](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_GetDatastore.html)

- &AHI; DICOM import jobs
    - [Starting a &AHI; DICOM import job](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_StartDICOMImportJob.html)
    - [Getting a &AHI; DICOM import job](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_GetDICOMImportJob.html)
    - [Listing &AHI; DICOM import jobs](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_ListDICOMImportJobs.html)

- &AHI; image sets  
    - [Searching &AHI; image sets](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_SearchImageSets.html)
    - [Getting an &AHI; image set](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_GetImageSet.html)
    - [Getting &AHI; image set metadata](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_GetImageSetMetadata.html)
    - [Getting an &AHI; image frame](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_GetImageFrame.html)
    - [Listing &AHI; image set versions](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_ListImageSetVersions.html)
    - [Updating &AHI; image set metadata](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_UpdateImageSetMetadata.html)
    - [Copying an &AHI; image set](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_CopyImageSet.html)
    - [Deleting an &AHI; image set](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_DeleteImageSet.html)

- &AHI; resource tagging
    - [Tagging an &AHI; resource](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_TagResource.html)
    - [Untagging an &AHI; resource](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_UntagResource.html) 
    - [Listing tags for an &AHI; resource](https://docs.aws.amazon.com/healthlake/latest/APIReference/API_ListTagsForResource.html)