resource "aws_appautoscaling_target" "ecs" {
  max_capacity       = local.service.scale_down_desired_count
  min_capacity       = local.service.scale_down_desired_count
  resource_id        = "service/${local.service.cluster_name}/${aws_ecs_service.service.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  service_namespace  = "ecs"
}

resource "aws_appautoscaling_scheduled_action" "scale_up" {
  name               = "${local.service.cluster_name}-${aws_ecs_service.service.name}-scale_up"
  service_namespace  = aws_appautoscaling_target.ecs.service_namespace
  resource_id        = aws_appautoscaling_target.ecs.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs.scalable_dimension
  schedule           = local.service.scale_up_cron # ScaleUp cron 작성 기준 UTC / 한국시간 오후 8시 00분 / 시카고 시간 오전 6시 00분

  scalable_target_action {
    min_capacity = local.service.scale_up_desired_count
    max_capacity = local.service.scale_up_desired_count
  }
}

resource "aws_appautoscaling_scheduled_action" "scale_down" {
  name               = "${local.service.cluster_name}-${aws_ecs_service.service.name}-scale_down"
  service_namespace  = aws_appautoscaling_target.ecs.service_namespace
  resource_id        = aws_appautoscaling_target.ecs.resource_id
  scalable_dimension = aws_appautoscaling_target.ecs.scalable_dimension
  schedule           = local.service.scale_down_cron # ScaleUp cron 작성 기준 UTC / 한국시간 오전 9시 00분 / 시카고 시간 오후 7시 00분

  scalable_target_action {
    min_capacity = local.service.scale_down_desired_count
    max_capacity = local.service.scale_down_desired_count
  }
}
