https://docs.aws.amazon.com/cognito/latest/developerguide/getting-credentials.html

Use STS to trade an identity for a credential

https://us-east-1.console.aws.amazon.com/cognito/v2/idp/user-pools/create?region=us-east-1

Default options, then: Email; no MFA; SES configured email address, name "WASM Demo", app client name "WASM Demo", generate client secret. Create Cognito Domain (wasm-demo-$(date | shasum | cut -c 1-8)). Edit hosted ui: callback url http://localhost:3000, Identity provider Cognito user pool, OAuth grant type Implicit grant, Connect scopes OpenID Email and aws.cognito.signin.user.admin.

https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-app-integration.html

Set up hosted auth.

Do we also need an identity pool?

Name: WASM ID Pool. Authentication Providers Cognito User pool and Client id from user pool.

New Role. Trusted Entity: Web Identity. Use Identity pool. Attach policy with lambda:ListFunctions.

A user from this identity pool:
https://us-east-1.console.aws.amazon.com/cognito/pool/?region=us-east-1&id=us-east-1:8de5c99b-81fa-4914-8376-fd6f185023c7

needs to get to this role:
https://us-east-1.console.aws.amazon.com/iamv2/home#/roles/details/WASM-ListFunctions-Role?section=trust_relationships
