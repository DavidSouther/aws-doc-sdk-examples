/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

/*
The arithmetic handler is more complex:
1. It accepts a set of actions ['plus', 'minus', 'times', 'divided-by'] and two numbers, and returns the result of the calculation.
2. It uses an environment variable to control log level (such as DEBUG, INFO, WARNING, ERROR).
It logs a few things at different levels, such as:
    * DEBUG: full event data
    * INFO: the result of the calculation
    * WARN~ING~: when a divide by zero error occurs
    * This will be the typical `RUST_LOG` variable.
 */

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    Ok(())
}

use tracing::log::{debug, info, warn};

pub struct ArithmeticFunction {
    pub log_level: String,
}

impl ArithmeticFunction {
    pub fn new(log_level: String) -> Self {
        ArithmeticFunction { log_level }
    }

    pub fn calculate(&self, action: &str, num1: i32, num2: i32) -> Result<i32, anyhow::Error> {
        let result = match action {
            "plus" => Self::add(num1, num2),
            "minus" => Self::subtract(num1, num2),
            "times" => Self::multiply(num1, num2),
            "divided-by" => Self::divide(num1, num2),
            _ => Err(anyhow::anyhow!("Invalid action")),
        }?;

        debug!(
            "Full event data: action={}, num1={}, num2={}, result={}",
            action, num1, num2, result
        );
        info!("The result of the calculation: {}", result);

        Ok(result)
    }

    fn add(num1: i32, num2: i32) -> Result<i32, anyhow::Error> {
        Ok(num1 + num2)
    }

    fn subtract(num1: i32, num2: i32) -> Result<i32, anyhow::Error> {
        Ok(num1 - num2)
    }

    fn multiply(num1: i32, num2: i32) -> Result<i32, anyhow::Error> {
        Ok(num1 * num2)
    }

    fn divide(num1: i32, num2: i32) -> Result<i32, anyhow::Error> {
        if num2 == 0 {
            warn!("Attempted to divide by zero");
            return Err(anyhow::anyhow!("Cannot divide by zero"));
        }

        Ok(num1 / num2)
    }
}
