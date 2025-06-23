terraform {
  required_version = "1.12.2"
  backend "s3" {
    region = "ap-northeast-2"

    bucket  = "tbm20250628-infrastructure"
    key     = "infrastructure/terraform/main/prod/projects/application"
    encrypt = true

    dynamodb_table = "tbm20250628-terraform-lock"

  }
  required_providers {
    aws = {
      version = "6.0.0"
      source  = "hashicorp/aws"
    }
  }
}

provider "aws" {
  region              = "ap-northeast-2"
}
