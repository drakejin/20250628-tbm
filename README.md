# Terraform Infrastructure for TypeScript Backend Meetup

이 프로젝트는 **TypeScript Backend Meetup**에서 Terraform을 소개하기 위한 예제입니다. AWS ECS Fargate를 사용하여 컨테이너화된 API 서버를 배포하는 인프라를 구성합니다.

## 📋 목차

- [프로젝트 구조](#프로젝트-구조)
- [Terraform 기본 개념](#terraform-기본-개념)
- [인프라 아키텍처](#인프라-아키텍처)
- [사용법](#사용법)
- [주요 리소스](#주요-리소스)

## 🏗️ 프로젝트 구조

```
infrastructure/terraform/
├── README.md
├── init/                           # Terraform 백엔드 초기화
│   ├── main.tf                    # S3 버킷 및 DynamoDB 테이블 생성
│   └── conf.tf                    # Provider 설정
└── main/                          # 메인 인프라 코드
    ├── modules/                   # 재사용 가능한 모듈
    │   └── iam_role/             # IAM 역할 모듈
    │       ├── main.tf           # IAM 리소스 정의
    │       ├── var.tf            # 입력 변수
    │       └── out.tf            # 출력 값
    └── prod/                     # Production 환경
        ├── resources/            # 공통 리소스
        │   ├── service_ecs/      # ECS 클러스터
        │   └── service_lb/       # Application Load Balancer
        └── projects/             # 프로젝트별 리소스
            └── application/      # 애플리케이션 서비스
```

## 🎯 Terraform 기본 개념

이 프로젝트는 Terraform의 핵심 개념들을 실제 예제로 보여줍니다:

### 1. **Resource** - 인프라 리소스 정의

```hcl
# ECS 클러스터 리소스 생성
resource "aws_ecs_cluster" "ecs" {
  name = local.cluster_name
  tags = merge(local.tags, {
    Name     = local.cluster_name
    Cluster  = local.cluster_name
    Resource = "ecs"
  })
}
```

### 2. **Data** - 기존 리소스 참조

```hcl
# 기존 VPC 정보 조회
data "aws_vpc" "default" {
  id = "vpc-00b9015882d0f3f9e"
}

# 다른 Terraform 상태 참조
data "terraform_remote_state" "service_ecs" {
  backend = "s3"
  config = {
    bucket = "tbm20250628-infrastructure"
    key    = "infrastructure/terraform/main/prod/resources/service_ecs"
    region = "ap-northeast-2"
  }
}
```

### 3. **Output** - 다른 모듈에서 사용할 값 출력

```hcl
# ECS 클러스터 정보 출력
output "ecs_arn" {
  value = aws_ecs_cluster.ecs.arn
}

output "ecs_name" {
  value = aws_ecs_cluster.ecs.name
}
```

### 4. **Module** - 재사용 가능한 코드 블록

```hcl
# IAM 역할 모듈 사용
module "ecs_exec_role" {
  source = "../../../modules/iam_role"

  name = "${local.cluster_name}_${local.tags.Environment}_exec"
  tags = local.tags

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Action = "sts:AssumeRole"
      Effect = "Allow"
      Principal = {
        Service = "ecs-tasks.amazonaws.com"
      }
    }]
  })

  policy = jsonencode({
    # IAM 정책 내용...
  })
}
```

## 🏛️ 인프라 아키텍처

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Internet      │────│  Application     │────│   ECS Fargate   │
│   Gateway       │    │  Load Balancer   │    │   Tasks         │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │                         │
                              │                         │
                       ┌──────────────────┐    ┌─────────────────┐
                       │   Route53 DNS    │    │   CloudWatch    │
                       │   (도메인 연결)    │    │   Logs          │
                       └──────────────────┘    └─────────────────┘
```

### 주요 구성 요소:

1. **ECS Fargate**: 서버리스 컨테이너 실행 환경
2. **Application Load Balancer**: HTTP/HTTPS 트래픽 분산
3. **Route53**: DNS 관리 및 도메인 연결
4. **CloudWatch**: 로그 수집 및 모니터링
5. **Auto Scaling**: 트래픽에 따른 자동 확장/축소

## 🚀 사용법

### 1. 백엔드 초기화

```bash
cd infrastructure/terraform/init
terraform init
terraform plan
terraform apply
```

### 2. ECS 클러스터 생성

```bash
cd ../main/prod/resources/service_ecs
terraform init
terraform plan
terraform apply
```

### 3. Load Balancer 생성

```bash
cd ../service_lb
terraform init
terraform plan
terraform apply
```

### 4. 애플리케이션 배포

```bash
cd ../../projects/application
terraform init
terraform plan
terraform apply
```

## 📦 주요 리소스

### ECS 관련 리소스

- **ECS Cluster**: `tbm20250628-service-prod`
- **Task Definition**: Fargate 호환 컨테이너 정의
- **ECS Service**: 원하는 태스크 수 유지 및 로드밸런서 연결
- **Auto Scaling**: 시간 기반 스케일링 정책

### 네트워킹 리소스

- **Application Load Balancer**: HTTP(80) → HTTPS(443) 리다이렉트
- **Security Groups**: 포트 80, 443 허용
- **Target Groups**: ECS 태스크와 ALB 연결

### 보안 리소스

- **IAM Roles**: ECS 실행 및 태스크 역할
- **SSL Certificate**: ACM을 통한 HTTPS 지원

### 모니터링 리소스

- **CloudWatch Log Groups**: 컨테이너 로그 수집
- **Route53 Records**: 도메인과 ALB 연결

## 🔧 설정 가능한 값

### 컨테이너 설정

```hcl
container = {
  cpu          = 256        # CPU 단위 (1024 = 1 vCPU)
  memory       = 512        # 메모리 (MB)
  port         = 3000       # 컨테이너 포트
  repository   = "hashicorp/http-echo"  # Docker 이미지
  tag          = "latest"   # 이미지 태그
}
```

### 서비스 설정

```hcl
service = {
  default_desired_count = 1              # 기본 태스크 수
  scale_up_desired_count = 2             # 스케일 업 시 태스크 수
  scale_down_desired_count = 1           # 스케일 다운 시 태스크 수
  scale_up_cron = "cron(00 00 ? * SUN *)"    # 스케일 업 스케줄
  scale_down_cron = "cron(00 11 ? * SUN)"    # 스케일 다운 스케줄
}
```

## 📝 학습 포인트

이 예제를 통해 다음을 학습할 수 있습니다:

1. **모듈화**: 재사용 가능한 IAM 역할 모듈 구현
2. **상태 관리**: Remote State를 통한 리소스 간 데이터 공유
3. **환경 분리**: Production 환경 구성
4. **보안**: IAM 역할과 정책을 통한 최소 권한 원칙
5. **확장성**: Auto Scaling을 통한 자동 확장
6. **모니터링**: CloudWatch를 통한 로그 관리

## ⚠️ 주의사항

- **비용**: ECS Fargate, ALB, Route53 등은 사용량에 따라 비용이 발생합니다
- **도메인**: 예제에서는 `sundaytycoon.com` 도메인을 사용하므로, 실제 사용 시 본인 도메인으로 변경하세요
- **VPC**: 기본 VPC를 사용하고 있으므로, 프로덕션 환경에서는 전용 VPC 구성을 권장합니다
- **보안**: 하드코딩된 서브넷 ID와 보안 그룹 ID는 실제 환경에 맞게 수정하세요

---

이 프로젝트는 Terraform의 기본 개념을 실습하기 위한 교육용 예제입니다. 실제 프로덕션 환경에서는 보안, 네트워킹, 모니터링 등을 더욱 세밀하게 구성해야 합니다.
