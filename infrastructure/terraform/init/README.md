# Terraform Remote State 초기화

이 디렉토리는 Terraform의 remote state backend를 구성하기 위한 초기 설정을 담고 있습니다.

## 개요

Terraform의 remote state와 state locking을 위해 다음 AWS 리소스들을 생성합니다:

- **S3 버킷**: Terraform state 파일을 저장하는 remote backend 역할
- **DynamoDB 테이블**: Terraform state locking을 구현하여 동시 실행 방지

## 구성 요소

### 1. S3 버킷 (Remote State Backend)
```hcl
resource "aws_s3_bucket" "bucket" {
  bucket = "tbm20250628-infrastructure"
  # ... 태그 설정
}
```

- **역할**: Terraform state 파일(.tfstate)을 원격으로 저장
- **이점**:
  - 팀 협업 시 state 파일 공유
  - 로컬 state 파일 분실 위험 방지
  - 버전 관리 및 백업

### 2. DynamoDB 테이블 (State Locking)
```hcl
resource "aws_dynamodb_table" "terraform-lock" {
  name = "tbm20250628-terraform-lock"
  hash_key = "LockID"
  # ... 기타 설정
}
```

- **역할**: Terraform 실행 시 state locking 구현
- **이점**:
  - 동시에 여러 사용자가 terraform apply 실행하는 것을 방지
  - State 파일 corruption 방지
  - 안전한 인프라 변경 보장

## 사용 방법

### 1. 초기 설정
```bash
# terraform/init 디렉토리에서 실행
terraform init
terraform plan
terraform apply
```

### 2. 기존 리소스 Import (선택사항)
기존에 수동으로 생성된 S3 버킷이나 DynamoDB 테이블이 있다면:
```bash
./import-state.sh
```

### 3. 다른 Terraform 프로젝트에서 Remote Backend 사용
다른 Terraform 프로젝트의 `backend.tf` 또는 `main.tf`에 다음 설정 추가:

```hcl
terraform {
  backend "s3" {
    bucket         = "tbm20250628-infrastructure"
    key            = "path/to/your/terraform.tfstate"
    region         = "ap-northeast-2"
    dynamodb_table = "tbm20250628-terraform-lock"
    encrypt        = true
  }
}
```

## 주의사항

1. **Bootstrap 문제**: 이 init 프로젝트 자체는 local state를 사용해야 합니다 (remote backend를 생성하는 프로젝트이므로)
2. **권한**: AWS 계정에 S3, DynamoDB 리소스 생성 권한이 필요합니다
3. **리전**: 현재 `ap-northeast-2` (서울) 리전으로 설정되어 있습니다
4. **비용**: DynamoDB 테이블은 최소 용량(read/write capacity 1)으로 설정되어 있습니다

## 파일 구조

- `conf.tf`: Terraform 및 AWS provider 설정
- `main.tf`: S3 버킷과 DynamoDB 테이블 리소스 정의
- `import-state.sh`: 기존 리소스 import 스크립트
- `README.md`: 이 문서

## 태그 정책

모든 리소스에는 다음 태그가 적용됩니다:
- `Crew`: tbm20250628
- `Team`: platform
- `Service`: infrastructure
- `Repository`: tbm20250628/infrastructure

이를 통해 리소스 관리 및 비용 추적이 용이합니다.
