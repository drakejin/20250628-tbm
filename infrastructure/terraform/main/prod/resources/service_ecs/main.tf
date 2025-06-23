locals {
  tags = {
    Service     = "tbm20250628"
    Environment = "prod"
    Repository  = "github.com/drakejin/tbm20250628"
  }

  cluster_name = "tbm20250628-service-prod"
}

resource "aws_ecs_cluster" "ecs" {
  name = local.cluster_name
  tags = merge(local.tags, {
    Name     = local.cluster_name
    Cluster  = local.cluster_name
    Resource = "ecs"
  })
}

# ECS 를 실행하기위한 역할
module "ecs_exec_role" {
  source = "../../../modules/iam_role"

  tags = merge(local.tags, {
    Name = "${local.cluster_name}_${local.tags.Environment}_exec"
  })
  name = "${local.cluster_name}_${local.tags.Environment}_exec"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Effect": "Allow",
      "Principal": {
        "Service": "ecs-tasks.amazonaws.com"
      }
    }
  ]
}
EOF

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "ecr:GetAuthorizationToken",
        "ecr:BatchCheckLayerAvailability",
        "ecr:GetDownloadUrlForLayer",
        "ecr:BatchGetImage"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:logs:*"
      ]
    },
    {
      "Action": [
        "kms:Decrypt"
      ],
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": [
        "ssm:Describe*",
        "ssm:Get*",
        "ssm:List*",
        "secretsmanager:*"
      ],
      "Effect": "Allow",
      "Resource": "*"
    }
  ]
}
EOF
}