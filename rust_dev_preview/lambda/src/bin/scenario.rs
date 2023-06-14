/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

/*
## Service actions

Service Actions wrap the SDK call, taking a client and any specific parameters necessary for the call.

* CreateFunction
* GetFunction
* ListFunctions
* Invoke
* UpdateFunctionCode
* UpdateFunctionConfiguration
* DeleteFunction

## Scenario
A scenario runs at a command prompt and prints output to the user on the result of each service action. A scenario can run in one of two ways: straight through, printing out progress as it goes, or as an interactive question/answer script.

## Getting started with functions

Use an SDK to manage Lambda functions: create a function, invoke it, update its code, invoke it again, view its output and logs, and delete it.

This scenario uses two lambda handlers:
_Note: Handlers don't use AWS SDK API calls_

The increment handler is very simple:

1. It accepts a number, increments it, and returns the new value.
2. It performs simple logging of the result.

The arithmetic handler is more complex:
1. It accepts a set of actions ['plus', 'minus', 'times', 'divided-by'] and two numbers, and returns the result of the calculation.
2. It uses an environment variable to control log level (such as DEBUG, INFO, WARNING, ERROR).
It logs a few things at different levels, such as:
    * DEBUG: full event data
    * INFO: the result of the calculation
    * WARN~ING~: when a divide by zero error occurs
    * This will be the typical `RUST_LOG` variable.


The steps of the scenario are:

1. Create an IAM role that:
    * Has an assume_role policy that grants 'lambda.amazonaws.com' the 'sts:AssumeRole' action.
    * Attaches the 'arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole' managed role.
    * _You must wait for ~10 seconds after the role is created before you can use it!_
2. Create a function (CreateFunction) for the increment handler by packaging it as a zip and either:
    * Adding it with CreateFunction Code.ZipFile.
    * --or--
    * Uploading it to S3 and adding it with CreateFunction Code.S3Bucket/S3Key.
    * _Note: Zipping the file does not have to be done in code._
    * If you have a waiter, use it to wait until the function is active. Otherwise, call GetFunction until State is Active.
3. Invoke the function with a number and print the result.
4. Update the function (UpdateFunctionCode) to the arithmetic handler by packaging it as a zip and either:
    * Adding it with UpdateFunctionCode ZipFile
    * --or--
    * Uploading it to S3 and adding it with UpdateFunctionCode S3Bucket/S3Key.
5. Call GetFunction until Configuration.LastUpdateStatus is 'Successful' (or 'Failed').
6. Update the environment variable by calling UpdateFunctionConfiguration and pass it a log level, such as:
    * Environment={'Variables': {'RUST_LOG': 'TRACE'}}
7. Invoke the function with an action from the list and a couple of values. Include LogType='Tail' to get logs in the result. Print the result of the calculation and the log.
8. [Optional] Invoke the function to provoke a divide-by-zero error and show the log result.
9. List all functions for the account, using pagination (ListFunctions).
10. Delete the function (DeleteFunction).
11. Delete the role.

Each step should use the function created in Service Actions to abstract calling the SDK.
 */

use aws_sdk_lambda::types::Environment;
use clap::Parser;
use std::{collections::HashMap, path::PathBuf, str::from_utf8};
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

use lambda_code_examples::actions::{
    InvokeArgs::{Arithmetic, Increment},
    LambdaManager, Operation,
};

#[derive(Debug, Parser)]
pub struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    pub region: Option<String>,

    // The bucket to use for the FunctionCode
    #[structopt(short, long)]
    pub bucket: Option<String>,

    // The name the Lambda function
    #[structopt(short, long)]
    pub lambda_name: Option<String>,

    // The number to increment
    #[structopt(short, long, default_value = "12")]
    pub inc: i32,

    // The left operand
    #[structopt(long, default_value = "19")]
    pub num_a: i32,

    // The right operand
    #[structopt(long, default_value = "23")]
    pub num_b: i32,

    // The arithmetic operation
    #[structopt(short, long, default_value = "plus")]
    pub operation: Operation,
}

fn code_path(lambda: &str) -> PathBuf {
    PathBuf::from(format!("../target/lambda/{lambda}/bootstrap.zip"))
}

async fn main_block(
    opt: &Opt,
    manager: &LambdaManager,
    code_location: String,
) -> Result<(), anyhow::Error> {
    let invoke = manager.invoke(Increment(opt.inc)).await?;
    if let Some(payload) = invoke
        .payload()
        .cloned()
        .map(|b| String::from_utf8(b.into_inner()))
    {
        info!(?payload, "Invoked function configured as increment");
    }

    let update_code = manager
        .update_function_code(code_path("arithmetic"), code_location.clone())
        .await?;

    let code_sha256 = update_code.code_sha256().unwrap_or("Unknown SHA");
    info!(?code_sha256, "Updated function code with arithmetic.zip");

    let arithmetic_args = Arithmetic(opt.operation, opt.num_a, opt.num_b);
    let invoke = manager.invoke(arithmetic_args).await?;
    if let Some(payload) = invoke
        .payload()
        .cloned()
        .map(|b| String::from_utf8(b.into_inner()))
    {
        info!(?payload, "Invoked function configured as arithmetic");
    }

    let update = manager
        .update_function_configuration(
            Environment::builder()
                .set_variables(Some(HashMap::from([(
                    "RUST_LOG".to_string(),
                    "trace".to_string(),
                )])))
                .build(),
        )
        .await?;
    info!(?update, "Updated function configuration");

    let invoke = manager
        .invoke(Arithmetic(opt.operation, opt.num_a, opt.num_b))
        .await?;
    if let Some(payload) = invoke
        .payload()
        .cloned()
        .map(|b| String::from_utf8(b.into_inner()))
    {
        info!(
            ?payload,
            "Invoked function configured as arithmetic with increased logging"
        );
    }

    let invoke = manager
        .invoke(Arithmetic(Operation::DividedBy, opt.num_a, 0))
        .await?;

    if let Some(payload) = invoke.payload().map(|b| from_utf8(b.as_ref())) {
        info!(
            ?payload,
            "Invoked function configured as arithmetic triggering divide by zero"
        );
    }

    Ok::<(), anyhow::Error>(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let opt = Opt::parse();
    let manager = LambdaManager::load_from_env(opt.lambda_name.clone(), opt.bucket.clone()).await;

    let key = match manager.create_function(code_path("increment")).await {
        Ok(init) => {
            info!(?init, "Created function, initially with increment.zip");
            let run_block = main_block(&opt, &manager, init.clone()).await;
            info!(?run_block, "Finished running example, cleaning up");
            Some(init)
        }
        Err(err) => {
            warn!(?err, "Error happened when initializing function");
            None
        }
    };

    let delete = manager.delete_function(key).await;
    info!(?delete, "Deleted function & cleaned up resources");
}
