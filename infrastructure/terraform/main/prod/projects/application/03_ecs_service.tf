resource "aws_ecs_service" "service" {
  name = "${local.service.name}_${local.tags.Environment}"

  tags = merge(local.tags, {
    Name = "${local.service.name}_${local.tags.Environment}"
  })

  cluster                            = data.terraform_remote_state.service_ecs.outputs.ecs_id
  desired_count                      = local.service.default_desired_count
  deployment_maximum_percent         = local.service.deployment_maximum_percent
  deployment_minimum_healthy_percent = local.service.deployment_minimum_healthy_percent
  task_definition                    = aws_ecs_task_definition.td.arn
  scheduling_strategy                = "REPLICA"
  launch_type                        = "FARGATE"

  network_configuration {
    security_groups  = local.vpc.sg_ids
    subnets          = local.vpc.subnet_ids
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.tg.arn
    container_name   = local.container.name
    container_port   = local.container.port
  }

  # lifecycle {
  #   ignore_changes = [task_definition]
  # }

  depends_on = [aws_ecs_task_definition.td]
}

resource "aws_lb_target_group" "tg" {
  name = "${local.service.name}-${local.tags.Environment}-${local.container.port}"
  tags = merge(local.tags, {
    "Name" = "${local.service.name}-${local.tags.Environment}-${local.container.port}"
  })

  target_type          = "ip"
  port                 = local.container.port
  protocol             = "HTTP"
  vpc_id               = local.vpc.id
  deregistration_delay = 30

  health_check {
    protocol            = "HTTP"
    path                = local.container.health_check
    healthy_threshold   = 3
    unhealthy_threshold = 2
    interval            = 60
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_lb_listener_rule" "subdomain" {
  listener_arn = local.lb.listener_443_arn
  priority     = 101

  action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.tg.arn
  }

  condition {
    host_header {
      values = [local.service.domain]
    }
  }

  depends_on = [aws_lb_target_group.tg]
}


resource "aws_route53_record" "subdomain" {
  zone_id = local.route53_zone_id
  name    = local.service.domain
  type    = "A"

  alias {
    name                   = local.lb.dns_name
    zone_id                = local.lb.zone_id
    evaluate_target_health = true
  }
}
