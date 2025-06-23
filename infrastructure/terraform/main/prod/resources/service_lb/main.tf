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

  lb_name = "tbm20250628-service-prod"
}

resource "aws_lb" "lb" {
  name = local.lb_name

  internal           = false
  load_balancer_type = "application"
  security_groups = [
    aws_security_group.sg.id,
  ]
  subnets = local.vpc.subnet_ids

  enable_deletion_protection = true

  tags = merge(local.tags, {
    Name     = local.lb_name
    Resource = "alb"
  })
}

resource "aws_lb_listener" "listener_80_arn" {
  load_balancer_arn = aws_lb.lb.arn
  port              = "80"
  protocol          = "HTTP"

  default_action {
    type = "redirect"

    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
  tags = merge(local.tags, { "Name" = "https" })
}


# aws console 웹 사이트에서 가져온거
data "aws_acm_certificate" "acm" {
  domain      = "sundaytycoon.com" # 과거 쓰던거
  statuses = ["ISSUED"]
  key_types = ["RSA_2048"]
  most_recent = true
}


resource "aws_lb_listener" "listener_443" {
  load_balancer_arn = aws_lb.lb.arn
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = data.aws_acm_certificate.acm.arn

  default_action {
    type = "fixed-response"

    fixed_response {
      content_type = "application/json"
      message_body = "{ \"message\": \"set action about 443 port\" }"
      status_code  = "200"
    }
  }
  tags = merge(local.tags, { "Name" = "https" })
}


resource "aws_security_group" "sg" {
  name = "${local.lb_name}-service-lb"

  tags = merge(local.tags, {
    Name = "${local.lb_name}-service-lb"
  })

  description = "access for ECS services [${local.lb_name}]"
  vpc_id      = local.vpc.id

  ingress {
    security_groups = local.vpc.sg_ids
    self            = true
    from_port       = 0
    to_port         = 0
    protocol        = -1
  }

  ingress {
    from_port = 80
    to_port   = 80
    protocol  = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    self      = false
  }

  ingress {
    from_port = 443
    to_port   = 443
    protocol  = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
    self      = false
  }
  lifecycle {
    ignore_changes = [ingress, egress]
  }
  egress {
    from_port = 0
    to_port   = 0
    protocol  = -1
    cidr_blocks = ["0.0.0.0/0"]
  }
}
