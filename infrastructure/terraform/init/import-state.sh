#!/bin/bash

terraform import aws_s3_bucket.bucket hypurrquant-infrastructure
terraform import aws_dynamodb_table.terraform-lock hypurrquant-terraform-lock
