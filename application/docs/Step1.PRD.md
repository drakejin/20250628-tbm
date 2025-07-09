# TODO List App - Product Requirements Document (PRD)

## 📋 Product Overview

**Product Name**: Simple TODO List API
**Version**: 1.0
**Target Users**: 개발자, 생산성 도구 사용자
**Platform**: REST API (웹/모바일 클라이언트 지원)

## 🎯 Product Goals

### Primary Goals
- 사용자가 할 일을 쉽게 생성, 관리, 완료할 수 있는 API 제공
- 직관적이고 빠른 TODO 관리 경험
- 안정적이고 확장 가능한 백엔드 서비스

### Success Metrics
- API 응답 시간 < 200ms
- 99.9% 가용성
- 사용자 만족도 4.5/5.0 이상

## 👥 Target Users

### Primary Users
- **개발자**: API를 활용해 TODO 앱을 구축하려는 개발자
- **생산성 추구자**: 간단하고 빠른 할 일 관리를 원하는 사용자

### User Personas
**김개발 (Frontend Developer)**
- 나이: 28세
- 목표: 빠르고 안정적인 TODO API로 프론트엔드 앱 개발
- 페인포인트: 복잡한 API 문서, 느린 응답 속도

## 🔧 Core Features

### 1. TODO 관리 (MVP)
**기능 설명**: 기본적인 TODO CRUD 기능
- ✅ TODO 생성 (제목, 설명, 우선순위)
- ✅ TODO 목록 조회 (페이지네이션, 필터링)
- ✅ TODO 수정 (제목, 설명, 우선순위, 상태)
- ✅ TODO 삭제
- ✅ TODO 상태 변경 (pending → in_progress → completed)

**API Endpoints**:
```
POST   /api/v1/todos          # TODO 생성
GET    /api/v1/todos          # TODO 목록 조회
GET    /api/v1/todos/{id}     # TODO 상세 조회
PUT    /api/v1/todos/{id}     # TODO 수정
DELETE /api/v1/todos/{id}     # TODO 삭제
PATCH  /api/v1/todos/{id}/status  # 상태 변경
```

### 2. 사용자 관리 (MVP)
**기능 설명**: 기본적인 사용자 인증 및 관리
- ✅ 사용자 회원가입
- ✅ 로그인/로그아웃
- ✅ 사용자 정보 조회/수정

**API Endpoints**:
```
POST   /api/v1/auth/register  # 회원가입
POST   /api/v1/auth/login     # 로그인
POST   /api/v1/auth/logout    # 로그아웃
GET    /api/v1/users/profile  # 프로필 조회
PUT    /api/v1/users/profile  # 프로필 수정
```

### 3. 카테고리 관리 (V1.1)
**기능 설명**: TODO를 카테고리별로 분류
- ⏳ 카테고리 생성/수정/삭제
- ⏳ TODO에 카테고리 할당
- ⏳ 카테고리별 TODO 필터링

### 4. 협업 기능 (V2.0)
**기능 설명**: 팀 단위 TODO 관리
- ⏳ TODO 공유
- ⏳ 팀 멤버 초대
- ⏳ 댓글 기능

## 📊 Data Models

### User Entity
```rust
struct User {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

### TODO Entity
```rust
struct Todo {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    status: TodoStatus,
    priority: Priority,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

enum Priority {
    Low,
    Medium,
    High,
}
```

## 🔌 API Specifications

### TODO 생성 API
```http
POST /api/v1/todos
Content-Type: application/json
Authorization: Bearer {token}

{
  "title": "장보기",
  "description": "우유, 빵, 계란 사기",
  "priority": "medium",
  "due_date": "2025-07-15T10:00:00Z"
}

Response (201):
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "title": "장보기",
  "description": "우유, 빵, 계란 사기",
  "status": "pending",
  "priority": "medium",
  "due_date": "2025-07-15T10:00:00Z",
  "created_at": "2025-07-09T12:00:00Z",
  "updated_at": "2025-07-09T12:00:00Z"
}
```

### TODO 목록 조회 API
```http
GET /api/v1/todos?status=pending&priority=high&page=1&limit=20
Authorization: Bearer {token}

Response (200):
{
  "todos": [
    {
      "id": "123e4567-e89b-12d3-a456-426614174000",
      "title": "긴급 회의 준비",
      "status": "pending",
      "priority": "high",
      "due_date": "2025-07-09T14:00:00Z"
    }
  ],
  "total": 1,
  "page": 1,
  "limit": 20,
  "total_pages": 1
}
```

## 🛡️ Security Requirements

### Authentication
- JWT 토큰 기반 인증
- 토큰 만료 시간: 24시간
- Refresh Token 지원

### Authorization
- 사용자는 본인의 TODO만 접근 가능
- Admin 사용자는 모든 TODO 조회 가능

### Data Protection
- 비밀번호 bcrypt 해싱 (cost: 12)
- HTTPS 강제
- SQL Injection 방지 (파라미터화된 쿼리)
- Input validation (제목 최대 255자, 설명 최대 1000자)

## ⚡ Performance Requirements

### Response Time
- GET 요청: < 100ms (95th percentile)
- POST/PUT 요청: < 200ms (95th percentile)
- 복잡한 검색: < 500ms (95th percentile)

### Scalability
- 동시 사용자 1,000명 지원
- 사용자당 최대 10,000개 TODO 지원
- 페이지네이션으로 대용량 데이터 처리

### Database
- PostgreSQL 15+ 사용
- 인덱스 최적화 (user_id, status, created_at)
- 연결 풀링 (최대 20개 커넥션)

## 🧪 Testing Requirements

### Unit Tests
- Repository layer: 100% 커버리지
- Service layer: 95% 커버리지
- Handler layer: 90% 커버리지

### Integration Tests
- 모든 API 엔드포인트 테스트
- 인증/인가 시나리오 테스트
- 에러 케이스 테스트

### Test Scenarios
```rust
// 예시 테스트 케이스
#[test]
async fn test_create_todo_success() {
    // 정상적인 TODO 생성 테스트
}

#[test]
async fn test_create_todo_unauthorized() {
    // 인증되지 않은 사용자 접근 테스트
}

#[test]
async fn test_get_todos_with_filters() {
    // 필터링된 TODO 목록 조회 테스트
}
```

## 📚 Documentation Requirements

### API Documentation
- OpenAPI 3.0 스펙 자동 생성
- Swagger UI 제공 (/swagger-ui)
- 모든 엔드포인트에 예시 요청/응답

### Code Documentation
- 모든 public 함수에 rustdoc 주석
- 아키텍처 문서 (README.md)
- 설치 및 실행 가이드

## 🚀 Launch Plan

### Phase 1: MVP (4주)
- Week 1-2: 기본 CRUD API 개발
- Week 3: 인증/인가 구현
- Week 4: 테스트 및 문서화

### Phase 2: Enhancement (2주)
- Week 5: 카테고리 기능 추가
- Week 6: 성능 최적화 및 배포

### Phase 3: Advanced Features (4주)
- Week 7-8: 협업 기능 구현
- Week 9-10: 모니터링 및 운영 기능

## ✅ Success Criteria

### Technical Success
- [ ] 모든 API 엔드포인트 구현 완료
- [ ] 90% 이상 테스트 커버리지 달성
- [ ] OpenAPI 문서 100% 완성
- [ ] 성능 요구사항 충족

### Business Success
- [ ] 개발자 온보딩 시간 < 30분
- [ ] API 오류율 < 0.1%
- [ ] 사용자 피드백 4.0/5.0 이상

## 🔄 Future Enhancements

### V2.0 Features
- 실시간 알림 (WebSocket)
- 파일 첨부 기능
- 반복 TODO (recurring tasks)
- 통계 및 리포트

### V3.0 Features
- 모바일 앱 지원
- 오프라인 동기화
- AI 기반 TODO 추천