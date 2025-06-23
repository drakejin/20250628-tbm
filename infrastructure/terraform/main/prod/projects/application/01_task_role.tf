module "task_role" {
  source = "../../../modules/iam_role"

  tags = merge(local.tags, {
    Name = "${local.service.name}_${local.tags.Environment}_task"
  })
  name = "${local.service.name}_${local.tags.Environment}_task"

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
      "Action": "s3:*",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "ecr:*",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "ssm:*",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "cloudwatch:*",
      "Effect": "Allow",
      "Resource": "*"
    },
    {
      "Action": "logs:*",
      "Effect": "Allow",
      "Resource": "*"
    }
  ]
}
EOF

}