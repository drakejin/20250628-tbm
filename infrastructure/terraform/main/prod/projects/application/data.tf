# 직접 웹사이트 가서 arn 주소 따온것
# aws vpc에 대해 자세히 공부한 사람들은 vpc를 직접정의해서 쓰시고 vpc잘 모르면 그냥 default 쓰는걸 추천, 나중에 인프라 개발자가 와서 피땀흘려 옮겨줄것 임.
data "aws_vpc" "default" {
  id = "vpc-00b9015882d0f3f9e"
}

data "aws_subnet" "public_subnet_a" {
  id = "subnet-0a9500e1bf1d81a30"
}

data "aws_subnet" "public_subnet_b" {
  id = "subnet-0b8ac361238aae761"
}

data "aws_subnet" "public_subnet_c" {
  id = "subnet-093bb61b0fcc34ffe"
}

data "aws_subnet" "public_subnet_d" {
  id = "subnet-4fe9c613"
}

data "aws_security_group" "default" {
  id = "sg-037cc7308cdc4cd4a"
}

data "terraform_remote_state" "service_ecs" {
  backend = "s3"

  config = {
    bucket  = "tbm20250628-infrastructure"
    key     = "infrastructure/terraform/main/prod/resources/service_ecs"
    region  = "ap-northeast-2"
    encrypt = true
  }
}

data "terraform_remote_state" "service_lb" {
  backend = "s3"

  config = {
    bucket  = "tbm20250628-infrastructure"
    key     = "infrastructure/terraform/main/prod/resources/service_lb"
    region  = "ap-northeast-2"
    encrypt = true
  }
}

data "aws_route53_zone" "route53_zone" {
  name         = "hansarangcare.com." # 예전에 구매해둔것 예시일뿐입니다;
  private_zone = false
}

data "aws_region" "current" {}