import { AttributeType, Table } from "aws-cdk-lib/aws-dynamodb";
import { Construct } from "constructs";

export interface AppDatabaseProps {}

export class AppDatabase extends Construct {
  static readonly TABLE_NAME = "Comments";
  static readonly KEY = "comment_key";
  static readonly INDEX = "sentiment";
  table: Table;

  constructor(scope: Construct, {}: AppDatabaseProps = {}) {
    super(scope, "ddb");
    this.table = new Table(this, AppDatabase.TABLE_NAME, {
      partitionKey: { name: AppDatabase.KEY, type: AttributeType.STRING },
      sortKey: {
        name: AppDatabase.INDEX,
        type: AttributeType.STRING,
      },
    });
  }
}
