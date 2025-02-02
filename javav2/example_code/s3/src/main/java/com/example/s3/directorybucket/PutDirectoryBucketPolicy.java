// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package com.example.s3.directorybucket;

// snippet-start:[s3directorybuckets.java2.put_directory_bucket_policy.import]

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import software.amazon.awssdk.regions.Region;
import software.amazon.awssdk.services.s3.S3Client;
import software.amazon.awssdk.services.s3.model.PutBucketPolicyRequest;
import software.amazon.awssdk.services.s3.model.S3Exception;

import static com.example.s3.util.S3DirectoryBucketUtils.createDirectoryBucket;
import static com.example.s3.util.S3DirectoryBucketUtils.createS3Client;
import static com.example.s3.util.S3DirectoryBucketUtils.deleteDirectoryBucket;
import static com.example.s3.util.S3DirectoryBucketUtils.getAwsAccountId;
// snippet-end:[s3directorybuckets.java2.put_directory_bucket_policy.import]

/**
 * Before running this example:
 * <p>
 * The SDK must be able to authenticate AWS requests on your behalf. If you have
 * not configured
 * authentication for SDKs and tools, see
 * https://docs.aws.amazon.com/sdkref/latest/guide/access.html in the AWS SDKs
 * and Tools Reference Guide.
 * <p>
 * You must have a runtime environment configured with the Java SDK.
 * See
 * https://docs.aws.amazon.com/sdk-for-java/latest/developer-guide/setup.html in
 * the Developer Guide if this is not set up.
 * <p>
 * To use S3 directory buckets, configure a gateway VPC endpoint. This is the
 * recommended method to enable directory bucket traffic without
 * requiring an internet gateway or NAT device. For more information on
 * configuring VPC gateway endpoints, visit
 * https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-networking.html#s3-express-networking-vpc-gateway.
 * <p>
 * Directory buckets are available in specific AWS Regions and Zones. For
 * details on Regions and Zones supporting directory buckets, see
 * https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-networking.html#s3-express-endpoints.
 */

public class PutDirectoryBucketPolicy {
    private static final Logger logger = LoggerFactory.getLogger(PutDirectoryBucketPolicy.class);

    // snippet-start:[s3directorybuckets.java2.directory_bucket_put_bucket_policy.main]
    /**
     * Sets the following bucket policy for the specified S3 directory bucket.
     *<pre>
     * {
     *     "Version": "2012-10-17",
     *     "Statement": [
     *         {
     *             "Sid": "AdminPolicy",
     *             "Effect": "Allow",
     *             "Principal": {
     *                 "AWS": "arn:aws:iam::<ACCOUNT_ID>:root"
     *             },
     *             "Action": "s3express:*",
     *             "Resource": "arn:aws:s3express:us-west-2:<ACCOUNT_ID>:bucket/<DIR_BUCKET_NAME>
     *         }
     *     ]
     * }
     * </pre>
     * This policy grants all S3 directory bucket actions to identities in the same account as the bucket.
     *
     * @param s3Client   The S3 client used to interact with S3
     * @param bucketName The name of the directory bucket
     * @param policyText The policy text to be applied
     */
    public static void putDirectoryBucketPolicy(S3Client s3Client, String bucketName, String policyText) {
        logger.info("Setting policy on bucket: {}", bucketName);
        logger.info("Policy: {}", policyText);

        try {
            PutBucketPolicyRequest policyReq = PutBucketPolicyRequest.builder()
                    .bucket(bucketName)
                    .policy(policyText)
                    .build();

            s3Client.putBucketPolicy(policyReq);
            logger.info("Bucket policy set successfully!");

        } catch (S3Exception e) {
            logger.error("Failed to set bucket policy: {} - Error code: {}", e.awsErrorDetails().errorMessage(),
                    e.awsErrorDetails().errorCode(), e);
            throw e;
        }
    }
    // snippet-end:[s3directorybuckets.java2.directory_bucket_put_bucket_policy.main]

    // Main method for testing
    public static void main(String[] args) {
        Region region = Region.US_WEST_2;
        S3Client s3Client = createS3Client(region);
        String zone = "usw2-az1";
        String bucketName = "test-bucket-" + System.currentTimeMillis() + "--" + zone + "--x-s3";

        // Get AWS account ID
        String awsAccountId = getAwsAccountId();

        // Policy text
        String policyText = "{\n" +
                "    \"Version\": \"2012-10-17\",\n" +
                "    \"Statement\": [\n" +
                "        {\n" +
                "            \"Sid\": \"AdminPolicy\",\n" +
                "            \"Effect\": \"Allow\",\n" +
                "            \"Principal\": {\n" +
                "                \"AWS\": \"arn:aws:iam::" + awsAccountId + ":root\"\n" +
                "            },\n" +
                "            \"Action\": \"s3express:*\",\n" +
                "            \"Resource\": \"arn:aws:s3express:us-west-2:" + awsAccountId + ":bucket/" + bucketName
                + "\"\n" +
                "        }\n" +
                "    ]\n" +
                "}";

        try {
            // Create the directory bucket
            createDirectoryBucket(s3Client, bucketName, zone);
            // Put Bucket Policy
            putDirectoryBucketPolicy(s3Client, bucketName, policyText);
        } catch (S3Exception e) {
            logger.error("An error occurred during S3 operations: {} - Error code: {}",
                    e.awsErrorDetails().errorMessage(), e.awsErrorDetails().errorCode(), e);
        } finally {
            try {
                // Tear down by deleting the bucket
                deleteDirectoryBucket(s3Client, bucketName);
            } catch (S3Exception e) {
                logger.error("Failed to delete bucket: {} - Error code: {}", e.awsErrorDetails().errorMessage(),
                        e.awsErrorDetails().errorCode(), e);
            } catch (RuntimeException e) {
                logger.error("Failed to delete the bucket due to unexpected error: {}", e.getMessage(), e);
            } finally {
                s3Client.close();
            }
        }
    }
}
