# TODO List API 명세서

## 📋 API Overview

**Base URL**: `https://api.todoapp.com/api/v1`
**Authentication**: Bearer Token (JWT)
**Content-Type**: `application/json`
**API Version**: v1.0

## 🔐 Authentication

### 회원가입
```http
POST /auth/register
```

**Request Body**:
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "securepassword123"
}
```

**Response (201)**:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "user@example.com",
  "username": "johndoe",
  "created_at": "2025-07-09T12:00:00Z"
}
```

**Error Responses**:
- `400`: 잘못된 요청 데이터
- `409`: 이미 존재하는 이메일/사용자명

---

### 로그인
```http
POST /auth/login
```

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response (200)**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "user@example.com",
    "username": "johndoe"
  }
}
```

**Error Responses**:
- `401`: 잘못된 인증 정보
- `422`: 유효성 검사 실패

---

### 로그아웃
```http
POST /auth/logout
Authorization: Bearer {token}
```

**Response (204)**: No Content

---

## 👤 User Management

### 프로필 조회
```http
GET /users/profile
Authorization: Bearer {token}
```

**Response (200)**:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "user@example.com",
  "username": "johndoe",
  "created_at": "2025-07-09T12:00:00Z",
  "updated_at": "2025-07-09T12:00:00Z"
}
```

---

### 프로필 수정
```http
PUT /users/profile
Authorization: Bearer {token}
```

**Request Body**:
```json
{
  "username": "newusername",
  "email": "newemail@example.com"
}
```

**Response (200)**:
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "newemail@example.com",
  "username": "newusername",
  "updated_at": "2025-07-09T14:30:00Z"
}
```

---

## ✅ TODO Management

### TODO 생성
```http
POST /todos
Authorization: Bearer {token}
```

**Request Body**:
```json
{
  "title": "장보기",
  "description": "우유, 빵, 계란 구매",
  "priority": "medium",
  "due_date": "2025-07-15T10:00:00Z"
}
```

**Response (201)**:
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "title": "장보기",
  "description": "우유, 빵, 계란 구매",
  "status": "pending",
  "priority": "medium",
  "due_date": "2025-07-15T10:00:00Z",
  "created_at": "2025-07-09T12:00:00Z",
  "updated_at": "2025-07-09T12:00:00Z"
}
```

**Validation Rules**:
- `title`: 필수, 1-255자
- `description`: 선택, 최대 1000자
- `priority`: `low`, `medium`, `high` 중 하나
- `due_date`: ISO 8601 형식

---

### TODO 목록 조회
```http
GET /todos?status={status}&priority={priority}&page={page}&limit={limit}&search={search}
Authorization: Bearer {token}
```

**Query Parameters**:
- `status` (optional): `pending`, `in_progress`, `completed`
- `priority` (optional): `low`, `medium`, `high`
- `page` (optional): 페이지 번호 (기본값: 1)
- `limit` (optional): 페이지당 항목 수 (기본값: 20, 최대: 100)
- `search` (optional): 제목/설명에서 검색할 키워드

**Example Request**:
```http
GET /todos?status=pending&priority=high&page=1&limit=10
```

**Response (200)**:
```json
{
  "todos": [
    {
      "id": "456e7890-e89b-12d3-a456-426614174001",
      "title": "긴급 회의 준비",
      "description": "프레젠테이션 자료 작성",
      "status": "pending",
      "priority": "high",
      "due_date": "2025-07-09T14:00:00Z",
      "created_at": "2025-07-09T10:00:00Z",
      "updated_at": "2025-07-09T10:00:00Z"
    }
  ],
  "pagination": {
    "total": 25,
    "page": 1,
    "limit": 10,
    "total_pages": 3,
    "has_next": true,
    "has_prev": false
  }
}
```

---

### TODO 상세 조회
```http
GET /todos/{id}
Authorization: Bearer {token}
```

**Response (200)**:
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "title": "장보기",
  "description": "우유, 빵, 계란 구매",
  "status": "pending",
  "priority": "medium",
  "due_date": "2025-07-15T10:00:00Z",
  "created_at": "2025-07-09T12:00:00Z",
  "updated_at": "2025-07-09T12:00:00Z"
}
```

**Error Responses**:
- `404`: TODO를 찾을 수 없음
- `403`: 접근 권한 없음

---

### TODO 수정
```http
PUT /todos/{id}
Authorization: Bearer {token}
```

**Request Body**:
```json
{
  "title": "장보기 (수정됨)",
  "description": "우유, 빵, 계란, 과일 구매",
  "priority": "high",
  "due_date": "2025-07-14T10:00:00Z"
}
```

**Response (200)**:
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "title": "장보기 (수정됨)",
  "description": "우유, 빵, 계란, 과일 구매",
  "status": "pending",
  "priority": "high",
  "due_date": "2025-07-14T10:00:00Z",
  "created_at": "2025-07-09T12:00:00Z",
  "updated_at": "2025-07-09T15:30:00Z"
}
```

---

### TODO 상태 변경
```http
PATCH /todos/{id}/status
Authorization: Bearer {token}
```

**Request Body**:
```json
{
  "status": "in_progress"
}
```

**Response (200)**:
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174001",
  "status": "in_progress",
  "updated_at": "2025-07-09T16:00:00Z"
}
```

**Valid Status Transitions**:
- `pending` → `in_progress`
- `pending` → `completed`
- `in_progress` → `completed`
- `in_progress` → `pending`
- `completed` → `pending`

---

### TODO 삭제
```http
DELETE /todos/{id}
Authorization: Bearer {token}
```

**Response (204)**: No Content

---

## 📊 Statistics (V1.1)

### 통계 조회
```http
GET /todos/stats
Authorization: Bearer {token}
```

**Response (200)**:
```json
{
  "total_todos": 150,
  "completed_todos": 120,
  "pending_todos": 25,
  "in_progress_todos": 5,
  "completion_rate": 80.0,
  "overdue_todos": 3,
  "today_completed": 5,
  "this_week_completed": 25
}
```

---

## 🏷️ Categories (V1.1)

### 카테고리 목록 조회
```http
GET /categories
Authorization: Bearer {token}
```

**Response (200)**:
```json
{
  "categories": [
    {
      "id": "789e1234-e89b-12d3-a456-426614174002",
      "name": "업무",
      "color": "#FF5722",
      "todo_count": 15,
      "created_at": "2025-07-09T12:00:00Z"
    }
  ]
}
```

---

### 카테고리 생성
```http
POST /categories
Authorization: Bearer {token}
```

**Request Body**:
```json
{
  "name": "개인",
  "color": "#2196F3"
}
```

**Response (201)**:
```json
{
  "id": "789e1234-e89b-12d3-a456-426614174003",
  "name": "개인",
  "color": "#2196F3",
  "todo_count": 0,
  "created_at": "2025-07-09T17:00:00Z"
}
```

---

## 🛡️ Error Responses

### 공통 에러 형식
```json
{
  "error": "validation_error",
  "message": "제목은 필수 항목입니다",
  "details": {
    "field": "title",
    "code": "required"
  },
  "timestamp": "2025-07-09T12:00:00Z"
}
```

### HTTP Status Codes
- `200`: 성공
- `201`: 생성 성공
- `204`: 성공 (응답 본문 없음)
- `400`: 잘못된 요청
- `401`: 인증 필요
- `403`: 권한 없음
- `404`: 리소스를 찾을 수 없음
- `409`: 충돌 (중복 데이터)
- `422`: 유효성 검사 실패
- `429`: 요청 제한 초과
- `500`: 서버 내부 오류

---

## 📝 Data Types

### Enums
```typescript
// TODO 상태
enum TodoStatus {
  pending = "pending",
  in_progress = "in_progress",
  completed = "completed"
}

// 우선순위
enum Priority {
  low = "low",
  medium = "medium",
  high = "high"
}
```

### Date Format
- 모든 날짜는 ISO 8601 UTC 형식: `2025-07-09T12:00:00Z`

---

## 🔧 Rate Limiting

- **인증된 사용자**: 시간당 1000 요청
- **미인증 요청**: 시간당 100 요청 (회원가입/로그인만)

**Rate Limit Headers**:
```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1625846400
```

---

## 📚 Example Usage

### 일반적인 워크플로우

1. **회원가입/로그인**
```bash
curl -X POST https://api.todoapp.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password123"}'
```

2. **TODO 생성**
```bash
curl -X POST https://api.todoapp.com/api/v1/todos \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title":"새로운 할일","priority":"high"}'
```

3. **TODO 목록 조회**
```bash
curl -X GET "https://api.todoapp.com/api/v1/todos?status=pending" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

4. **TODO 완료 처리**
```bash
curl -X PATCH https://api.todoapp.com/api/v1/todos/{id}/status \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"status":"completed"}'
```

---

## 🔄 Versioning

- **현재 버전**: v1
- **지원 정책**: 마이너 버전은 하위 호환성 보장
- **Deprecation**: 6개월 전 공지 후 제거

**Version Header**:
```http
API-Version: v1
```