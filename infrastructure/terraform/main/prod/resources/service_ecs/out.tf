output "ecs_arn" {
  value = aws_ecs_cluster.ecs.arn
}

output "ecs_id" {
  value = aws_ecs_cluster.ecs.id
}

output "ecs_name" {
  value = aws_ecs_cluster.ecs.name
}

output "ecs_exec_role_arn" {
  value = module.ecs_exec_role.arn
}

output "ecs_exec_role_id" {
  value = module.ecs_exec_role.id
}