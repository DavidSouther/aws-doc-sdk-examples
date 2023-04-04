/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */
import { fromCognitoIdentityPool } from "@aws-sdk/credential-providers";
import { CognitoIdentityClient } from "@aws-sdk/client-cognito-identity";

import {
  STSClient,
  AssumeRoleWithWebIdentityCommand,
} from "@aws-sdk/client-sts";

import { LambdaClient, ListFunctionsCommand } from "@aws-sdk/client-lambda";

const EMPTY_CREDENTIALS = {
  accessKeyId: "",
  secretAccessKey: "",
  sessionToken: "",
};

const region = "us-east-1";

const HOSTED_UI_DOMAIN = "wasm-demo-a48ce63c.auth.us-east-1.amazoncognito.com";
const CLIENT_ID = "2al53frjfsb46uk7ujeju53vba";

const USER_POOL_ID = "us-east-1_tECk9W0Dk";
const RoleArn = "arn:aws:iam::659765859849:role/WASM-ListFunctions-Role";
const identityPoolId = "us-east-1:8de5c99b-81fa-4914-8376-fd6f185023c7";

const REDIRECT_URI = encodeURIComponent("http://localhost:3000/");
export const LOGIN_URL = `https://${HOSTED_UI_DOMAIN}/login?response_type=token&client_id=${CLIENT_ID}&redirect_uri=${REDIRECT_URI}`;

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("login").setAttribute("href", LOGIN_URL);
});

document.addEventListener("DOMContentLoaded", async () => {
  const url = new URL(document.URL);
  const hash = url.hash;
  const hashParams = new URLSearchParams(hash.substring(1));
  if (!hashParams.has("id_token")) return;

  const token = hashParams.get("id_token");
  const tokenDetails = JSON.parse(atob(token.split(".")[1]));
  console.log(tokenDetails);

  const cognitoidentity = new CognitoIdentityClient({
    region,
    credentials: fromCognitoIdentityPool({
      identityPoolId,
      userIdentifier: tokenDetails["email"],
      clientConfig: { region },
      logins: {
        [`cognito-idp.${region}.amazonaws.com/${USER_POOL_ID}`]: token,
      },
    }),
  });

  const cognitoCredentials = await cognitoidentity.config.credentials();
  console.log(cognitoCredentials);

  // const stsClient = new STSClient({ region, credentials: cognitoCredentials });
  // const assumedRole = await stsClient.send(
  //   new AssumeRoleWithWebIdentityCommand({
  //     RoleArn,
  //     RoleSessionName: tokenDetails["email"],
  //     WebIdentityToken: token,
  //   })
  // );

  const credentials = cognitoCredentials;

  EMPTY_CREDENTIALS.accessKeyId = credentials.accessKeyId;
  EMPTY_CREDENTIALS.secretAccessKey = credentials.secretAccessKey;
  EMPTY_CREDENTIALS.sessionToken = credentials.sessionToken;

  setCredentials(credentials);

  const lambdaClient = new LambdaClient({
    region,
    credentials,
  });

  const functions = await lambdaClient.send(new ListFunctionsCommand({}));
  console.log(functions);
});

const parseCookie = (str) =>
  str
    .split(";")
    .map((v) => v.split("="))
    .reduce((acc, v) => {
      if (v[0] && v[1]) {
        acc[decodeURIComponent(v[0].trim())] = decodeURIComponent(v[1].trim());
      }
      return acc;
    }, {});

export const retrieveCredentials = () => {
  let cookie = parseCookie(document.cookie ?? "");
  return cookie.credentials_aws
    ? JSON.parse(cookie.credentials_aws)
    : EMPTY_CREDENTIALS;
};

export const setCredentials = (credentials) => {
  const encodedCredentials = encodeURIComponent(
    JSON.stringify({
      ...credentials,
    })
  );
  document.cookie = `credentials_aws=${encodedCredentials}; max-age=43200; path=/;`;
};
