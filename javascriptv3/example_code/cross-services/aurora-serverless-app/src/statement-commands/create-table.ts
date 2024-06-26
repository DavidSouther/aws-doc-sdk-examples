// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
import { buildStatementCommand } from "./command-helper.js";

const command = buildStatementCommand(`
CREATE TABLE items (iditem VARCHAR(45), description VARCHAR(400), guide VARCHAR(45), status VARCHAR(400), username VARCHAR(45), archived TINYINT(4));
`);

export { command };
