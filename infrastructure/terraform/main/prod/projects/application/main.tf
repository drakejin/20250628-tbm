locals {
  tags = {
    Service     = "tbm20250628"
    Environment = "prod"
    Team        = "infra"
    Managed     = "infra"
    Owner       = "drakejin"
    Repository  = "github.com/drakejin/tbm20250628"
    ManagedBy   = "terraform"
  }
  vpc = {
    id = data.aws_vpc.default.id
    sg_ids = [
      data.aws_security_group.default.id,
    ]
    subnet_ids = [
      data.aws_subnet.public_subnet_a.id,
      data.aws_subnet.public_subnet_b.id,
      data.aws_subnet.public_subnet_c.id,
      data.aws_subnet.public_subnet_d.id,
    ]
  }

  lb = {
    dns_name         = data.terraform_remote_state.service_lb.outputs.dns_name
    arn              = data.terraform_remote_state.service_lb.outputs.arn
    zone_id          = data.terraform_remote_state.service_lb.outputs.zone_id
    listener_443_arn = data.terraform_remote_state.service_lb.outputs.listener_443_arn
  }

  route53_zone_id = data.aws_route53_zone.route53_zone.zone_id

  service = {
    cluster_id   = data.terraform_remote_state.service_ecs.outputs.ecs_id
    cluster_name = data.terraform_remote_state.service_ecs.outputs.ecs_name
    name         = "application"
    domain       = "application.${data.aws_route53_zone.route53_zone.name}"

    default_desired_count = 1

    scale_down_desired_count = 1
    scale_down_cron          = "cron(00 11 ? * SUN)" # ScaleUp cron 작성 기준 UTC / 한국시간 오전 9시 00분 / 시카고 시간 오후 7시 00분

    scale_up_desired_count = 2
    scale_up_cron          = "cron(00 00 ? * SUN *)" # ScaleUp cron 작성 기준 UTC / 한국시간 오후 8시 00분 / 시카고 시간 오전 6시 00분

    deployment_maximum_percent         = 200
    deployment_minimum_healthy_percent = 100
  }


  container = {
    cpu          = 256
    memory       = 512
    health_check = "/"

    name = local.service.name
    port = 3000


    # repository = aws_ecr_repository.ecr_application.repository_url
    # tag  = "v1.0.0"
    repository = "hashicorp/http-echo"
    tag  = "latest"

  }
}
