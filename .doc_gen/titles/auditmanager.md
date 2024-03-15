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
  "auditmanager: {AssociateAssessmentReportEvidenceFolder: '',
  BatchDisassociateAssessmentReportEvidence: '', CreateAssessmentReport: '',
  DisassociateAssessmentReportEvidenceFolder: '', GetAssessment: '',
  GetAssessmentReportUrl: '', GetEvidenceFoldersByAssessment: '',
  GetEvidenceByEvidenceFolder: '', ListAssessmentReports: '', CreateControl: '',
  CreateAssessmentFramework: '', ListControls: '', GetControl: ''} Entities:
  AMlong AM"
---
- &AMlong; Controls: [Creating a Control](&AMlong;_CreateControl.html)
- &AMlong; Assessment Frameworks: [Creating an Assessment Framework](&AMlong;_CreateAssessmentFramework.html)
- &AMlong; Assessment Reports: 
  - [Associating an Assessment Report Evidence Folder](&AMlong;_AssociateAssessmentReportEvidenceFolder.html)
  - [Batch Disassociating Assessment Report Evidence](&AMlong;_BatchDisassociateAssessmentReportEvidence.html)
  - [Creating an Assessment Report](&AMlong;_CreateAssessmentReport.html)
  - [Disassociating an Assessment Report Evidence Folder](&AMlong;_DisassociateAssessmentReportEvidenceFolder.html)
  - [Getting an Assessment Report URL](&AMlong;_GetAssessmentReportUrl.html)
  - [Listing Assessment Reports](&AMlong;_ListAssessmentReports.html)
- &AMlong; Assessments:
  - [Getting an Assessment](&AMlong;_GetAssessment.html)
- &AMlong; Evidence:
  - [Getting Evidence Folders by Assessment](&AMlong;_GetEvidenceFoldersByAssessment.html)  
  - [Getting Evidence by Evidence Folder](&AMlong;_GetEvidenceByEvidenceFolder.html)
- &AMlong; Controls: [Listing Controls](&AMlong;_ListControls.html)
- &AMlong; Controls: [Getting a Control](&AMlong;_GetControl.html)