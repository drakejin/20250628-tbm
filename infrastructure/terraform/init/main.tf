resource "aws_s3_bucket" "bucket" {
  bucket = "tbm20250628-infrastructure"

  tags = {
    Crew       = "tbm20250628"
    Team       = "infra"
    Service    = "infrastructure"
    Repository = "tbm20250628"
  }
}

resource "aws_s3_bucket_ownership_controls" "ownership" {
  bucket = aws_s3_bucket.bucket.id
  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}


resource "aws_dynamodb_table" "terraform-lock" {
  name = "tbm20250628-terraform-lock"

  read_capacity  = 1
  write_capacity = 1

  hash_key = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }
}

