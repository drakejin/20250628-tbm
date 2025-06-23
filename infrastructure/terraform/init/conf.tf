terraform {
  required_version = "1.12.2"

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
