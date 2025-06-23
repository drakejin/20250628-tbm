resource "aws_ecs_task_definition" "td" {
  family             = "${local.container.name}_${local.tags.Environment}"
  task_role_arn      = module.task_role.arn
  execution_role_arn = data.terraform_remote_state.service_ecs.outputs.ecs_exec_role_arn
  tags = merge(local.tags, {
    "Name" = "${local.container.name}_${local.tags.Environment}"
  })

  requires_compatibilities = ["FARGATE"]
  network_mode             = "awsvpc"
  cpu                      = local.container.cpu
  memory                   = local.container.memory
  container_definitions    = jsonencode([
    {
      cpu : local.container.cpu,
      memory : local.container.memory,
      image : "${local.container.repository}:${local.container.tag}",
      name : local.container.name,
      essential : true,
      portMappings : [
        {
          hostPort : local.container.port,
          protocol : "tcp",
          containerPort : local.container.port
        }
      ],
      command : ["-listen=:${local.container.port}", "-text=TBM20250628"],
      healthCheck : {
        command : [
          "CMD",
          "/http-echo",
          "-version"
        ],
        retries : 3,
        timeout : 2,
        interval : 5,
        startPeriod : 2
      },
      environment : [
        {
          name : "TZ",
          value : "Etc/UTC"
        },
        {
          name : "ENV",
          value : local.tags.Environment
        },
        {
          name : "AWS_REGION",
          value : data.aws_region.current.region
        },
        {
          name : "AWS_DEFAULT_REGION",
          value : data.aws_region.current.region
        }
      ],
      logConfiguration : {
        logDriver : "awslogs",
        options : {
          "awslogs-group" : "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}",
          "awslogs-region" : data.aws_region.current.region,
          "awslogs-stream-prefix" : "/ecs/${local.tags.Environment}/${local.service.name}"
        }
      }
    }
  ])
}

resource "aws_cloudwatch_log_group" "log_group" {
  name = "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}"

  tags = merge(local.tags, {
    "Name" = "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}"
  })
}

# resource "aws_ecs_task_definition" "td" {
#   family             = "${local.container.name}_${local.tags.Environment}"
#   task_role_arn      = module.task_role.arn
#   execution_role_arn = data.terraform_remote_state.service_ecs.outputs.ecs_exec_role_arn
#   tags = merge(local.tags, {
#     "Name" = "${local.container.name}_${local.tags.Environment}"
#   })

#   requires_compatibilities = ["FARGATE"]
#   network_mode             = "awsvpc"
#   cpu                      = local.container.cpu
#   memory                   = local.container.memory
#   container_definitions    = jsonencode([
#     {
#       cpu : local.container.cpu,
#       memory : local.container.memory,
#       image : "${local.container.repository}:${local.container.tag}",
#       name : local.container.name,
#       essential : true,
#       portMappings : [
#         {
#           hostPort : local.container.port,
#           protocol : "tcp",
#           containerPort : local.container.port
#         }
#       ],
#       command : ["-listen=:${local.container.port}", "TBM20250628"],
#       healthCheck : {
#         command : [
#           "CMD-SHELL",
#           "curl -f http://0.0.0.0:${local.container.port}${local.container.health_check} || exit 1"
#         ],
#         retries : 5,
#         timeout : 60,
#         interval : 10,
#         startPeriod : 10
#       },
#       environment : [
#         {
#           name : "TZ",
#           value : "Etc/UTC"
#         },
#         {
#           name : "ENV",
#           value : local.tags.Environment
#         },
#         {
#           name : "AWS_REGION",
#           value : data.aws_region.current.region
#         },
#         {
#           name : "AWS_DEFAULT_REGION",
#           value : data.aws_region.current.region
#         }
#       ],
#       logConfiguration : {
#         logDriver : "awslogs",
#         options : {
#           "awslogs-group" : "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}",
#           "awslogs-region" : data.aws_region.current.region,
#           "awslogs-stream-prefix" : "/ecs/${local.tags.Environment}/${local.service.name}"
#         }
#       }
#     }
#   ])
# }

# resource "aws_cloudwatch_log_group" "log_group" {
#   name = "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}"

#   tags = merge(local.tags, {
#     "Name" = "/ecs/${local.tags.Environment}/${local.service.name}/${local.container.name}"
#   })
# }
