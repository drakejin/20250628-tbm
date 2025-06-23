locals {
  application = {
    organization = "tbm20250628"
    github       = "github.com/drakejin/tbm20250628"
    service_name = "application"
  }
}


resource "aws_ecr_repository" "ecr_application" {
  name                 = "${local.application.organization}/${local.application.service_name}"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = false
  }

  tags = merge(local.tags, {
    Service    = local.application.service_name
    Repository = local.application.github
  })
}


resource "aws_ecr_lifecycle_policy" "ecr_application_policy" {
  repository = aws_ecr_repository.ecr_application.name

  policy = <<EOF
{
    "rules": [
        {
            "rulePriority": 1,
            "description": "Keep last 100 images",
            "selection": {
                "tagStatus": "tagged",
                "tagPatternList": ["*"],
                "countType": "imageCountMoreThan",
                "countNumber": 100
            },
            "action": {
                "type": "expire"
            }
        }
    ]
}
EOF
}
