// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// snippet-start:[s3.go.delete_bucket]
package main

// snippet-start:[s3.go.delete_bucket.imports]
import (
	"flag"
	"fmt"

	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/s3"
)

// snippet-end:[s3.go.delete_bucket.imports]

// RemoveBucket deletes a bucket
// Inputs:
//
//	sess is the current session, which provides configuration for the SDK's service clients
//	bucket is the name of the bucket
//
// Output:
//
//	If success, nil
//	Otherwise, an error from the call to CreateBucket
func RemoveBucket(sess *session.Session, bucket *string) error {
	// snippet-start:[s3.go.delete_bucket.call]
	svc := s3.New(sess)

	_, err := svc.DeleteBucket(&s3.DeleteBucketInput{
		Bucket: bucket,
	})
	// snippet-end:[s3.go.delete_bucket.call]
	if err != nil {
		return err
	}

	// snippet-start:[s3.go.delete_bucket.wait]
	err = svc.WaitUntilBucketNotExists(&s3.HeadBucketInput{
		Bucket: bucket,
	})
	// snippet-end:[s3.go.delete_bucket.wait]
	if err != nil {
		return err
	}

	return nil
}

func main() {
	// snippet-start:[s3.go.delete_bucket.args]
	bucket := flag.String("b", "", "The name of the bucket")
	flag.Parse()

	if *bucket == "" {
		fmt.Println("You must supply a bucket name (-b BUCKET)")
		return
	}
	// snippet-end:[s3.go.delete_bucket.args]

	// snippet-start:[s3.go.delete_bucket.imports.session]
	sess := session.Must(session.NewSessionWithOptions(session.Options{
		SharedConfigState: session.SharedConfigEnable,
	}))
	// snippet-end:[s3.go.delete_bucket.imports.session]

	err := RemoveBucket(sess, bucket)
	if err != nil {
		fmt.Println("Could not delete bucket " + *bucket)
	}
}

// snippet-end:[s3.go.delete_bucket]
