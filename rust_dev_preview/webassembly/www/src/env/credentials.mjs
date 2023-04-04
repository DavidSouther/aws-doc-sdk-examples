/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */
import {
  STSClient,
  AssumeRoleWithWebIdentityCommand,
} from "@aws-sdk/client-sts";

const EXAMPLE_CREDENTIALS = {
  accessKeyId: "ASIAZTHI4CYE7YBRKR5L",
  secretAccessKey: "D6TuhJjB3Od6dsCWhZ4brH+FcamD4uxaBzSIvXXl",
  sessionToken:
    "IQoJb3JpZ2luX2VjENb//////////wEaCXVzLWVhc3QtMSJHMEUCIGWvBNQqEWYWa7AvMBjINaJyx0FPLlMznp+HmvIFfL51AiEAozpqP9AnTJkb3jBezGi0MIUbCiNEqUcMLy0J3q4VGxIqpgIIr///////////ARAAGgw2NTk3NjU4NTk4NDkiDG+JtSSk9iOKW5P33yr6AetUQxW3JVCTQ4rHPaB1hX8fWeqBvbUztrE+Za49pmETDsffyv5lXoiq8xvMQbjsymIhTa/lezM76+QlZn5jV021jv3hC2e6JI0QCN87n94OAN+dfJ9n+eP0OeGFllXWCbixuHVK7qXBIZY+1Ni9fkZ6ts5SqSmfwDz0jPwSBHmgStqPoN6BOsew+dGbkM7MujIUHZIdMeicZdliUygvnzC6WQDY+rJEcCzSkAbf87QpYrIoHmR2iHdhFAmrkyOJ3+Ts7r5b9t3A5ZWEbT0klaDug+67YNaQKtoaUuRma5FtXCWQakr7iIIhTLxhIdFI2eso3Tborc1+8Yww8MWioQY6nQGJluUe8g1u0fTH0IfWgOMrelfoZuT/1C++RsnZqX1bvRvXGtjlAxitwUyau85MeIo4xKV5mD51MeRe0XK7VjYCseaR8uq5+5aAEv62CUQ1Xp12NLQHEAJEkKGnusuuGlwv+ZmufFuKjjOwlnIpmaH2nWcSg1HsyF4zFaXeaKTg7YLS9m32JSUE7CbTwxAYYZXlSUbr8o80TYgQ9BVn",
};

const HOSTED_UI_DOMAIN = "wasm-demo-a48ce63c.auth.us-east-1.amazoncognito.com";
const CLIENT_UD = "2al53frjfsb46uk7ujeju53vba";
const RoleArn = "arn:aws:iam::659765859849:role/WASM-ListFunctions-Role";

const REDIRECT_URI = encodeURIComponent("http://localhost:3000/");
const region = "us-east-1";
export const LOGIN_URL = `https://${HOSTED_UI_DOMAIN}/login?response_type=token&client_id=${CLIENT_UD}&redirect_uri=${REDIRECT_URI}`;

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("login").setAttribute("href", LOGIN_URL);
});

document.addEventListener("DOMContentLoaded", () => {
  const url = new URL(document.URL);
  const hash = url.hash;
  const hashParams = new URLSearchParams(hash.substring(1));
  if (!hashParams.has("id_token")) return;

  const token = hashParams.get("id_token");
  const tokenDetails = JSON.parse(atob(token.split(".")[1]));
  console.log(tokenDetails);

  const client = new STSClient({ region });

  const command = new AssumeRoleWithWebIdentityCommand({
    RoleArn,
    RoleSessionName: tokenDetails["email"],
    WebIdentityToken: token,
  });

  client
    .send(command)
    .then((data) => {
      console.log("Assumed role", data);
    })
    .catch((e) => console.error("Failed to assume role:", e));
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
    : EXAMPLE_CREDENTIALS;
};

export const setCredentials = () => {
  const credentials = encodeURIComponent(
    JSON.stringify({
      ...EXAMPLE_CREDENTIALS,
    })
  );
  document.cookie = `credentials_aws=${credentials}; max-age=43200; path=/;`;
};
