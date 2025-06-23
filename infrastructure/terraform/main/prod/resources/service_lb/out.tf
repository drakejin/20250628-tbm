output "dns_name" {
  value = aws_lb.lb.dns_name
}

output "arn" {
  value = aws_lb.lb.arn
}

output "zone_id" {
  value = aws_lb.lb.zone_id
}

output "listener_443_arn" {
  value = aws_lb_listener.listener_443.arn
}