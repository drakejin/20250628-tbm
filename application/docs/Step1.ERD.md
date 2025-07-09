# TODO List App - Entity Relationship Diagram (ERD)

## 📊 Database Schema

**Database**: PostgreSQL 15+
**Character Set**: UTF8
**Collation**: utf8_general_ci

## 🗃️ Tables

### 1. users (사용자 테이블)

사용자 정보를 저장하는 메인 테이블입니다.

**필드 구성:**
- `id` (UUID): 사용자 고유 식별자 (UUID v4 형식)
- `email` (VARCHAR(255)): 사용자 이메일 주소 (로그인 ID로 사용, 중복 불가)
- `username` (VARCHAR(100)): 사용자명 (화면 표시용, 중복 불가, 영문/숫자/언더스코어만 허용)
- `password_hash` (VARCHAR(255)): 비밀번호 해시값 (bcrypt 알고리즘 사용, cost=12)
- `created_at` (TIMESTAMPTZ): 계정 생성 일시 (UTC 기준)
- `updated_at` (TIMESTAMPTZ): 계정 정보 최종 수정 일시 (UTC 기준)

**인덱스:**
- `idx_users_email`: 이메일 검색용 유니크 인덱스 (로그인 시 사용)
- `idx_users_username`: 사용자명 검색용 유니크 인덱스 (중복 체크 및 검색용)
- `idx_users_created_at`: 생성일시 기준 정렬용 인덱스 (관리자 페이지에서 사용)

### 2. todos (할일 테이블)

사용자의 할일 정보를 저장하는 메인 테이블입니다.

**필드 구성:**
- `id` (UUID): 할일 고유 식별자 (UUID v4 형식)
- `user_id` (UUID): 할일 소유자의 사용자 ID (users.id와 연관, 외래키 제약조건 없음)
- `title` (VARCHAR(255)): 할일 제목 (필수 입력, 최대 255자, 검색 대상)
- `description` (TEXT): 할일 상세 설명 (선택 입력, 최대 1000자, 검색 대상)
- `status` (VARCHAR(20)): 할일 진행 상태 (pending: 대기중, in_progress: 진행중, completed: 완료)
- `priority` (VARCHAR(10)): 할일 우선순위 (low: 낮음, medium: 보통, high: 높음)
- `due_date` (TIMESTAMPTZ): 할일 마감 일시 (UTC 기준, 선택 입력)
- `created_at` (TIMESTAMPTZ): 할일 생성 일시 (UTC 기준)
- `updated_at` (TIMESTAMPTZ): 할일 최종 수정 일시 (UTC 기준)

**인덱스:**
- `idx_todos_user_id`: 사용자별 할일 조회용 인덱스 (가장 자주 사용되는 쿼리)
- `idx_todos_status`: 상태별 할일 조회용 인덱스 (필터링에 사용)
- `idx_todos_priority`: 우선순위별 할일 조회용 인덱스 (필터링에 사용)
- `idx_todos_created_at`: 생성일시 기준 정렬용 인덱스 (최신순 정렬에 사용)
- `idx_todos_due_date`: 마감일시 기준 정렬용 인덱스 (마감일 임박 순서로 정렬)
- `idx_todos_user_status`: 사용자별 상태 조회용 복합 인덱스 (성능 최적화)
- `idx_todos_user_priority`: 사용자별 우선순위 조회용 복합 인덱스 (성능 최적화)
- `idx_todos_title_search`: 제목 검색용 인덱스 (전문 검색 기능)
- `idx_todos_description_search`: 설명 검색용 인덱스 (전문 검색 기능)

### 3. categories (카테고리 테이블) - V1.1

할일 분류를 위한 카테고리 테이블입니다. (V1.1에서 추가)

**필드 구성:**
- `id` (UUID): 카테고리 고유 식별자 (UUID v4 형식)
- `user_id` (UUID): 카테고리 소유자의 사용자 ID (users.id와 연관, 외래키 제약조건 없음)
- `name` (VARCHAR(100)): 카테고리 이름 (사용자별 중복 불가, 최대 100자)
- `color` (VARCHAR(7)): 카테고리 색상 코드 (HEX 형식, 예: #FF5722)
- `created_at` (TIMESTAMPTZ): 카테고리 생성 일시 (UTC 기준)
- `updated_at` (TIMESTAMPTZ): 카테고리 최종 수정 일시 (UTC 기준)

**인덱스:**
- `idx_categories_user_id`: 사용자별 카테고리 조회용 인덱스
- `idx_categories_user_name`: 사용자별 카테고리명 중복 방지용 복합 유니크 인덱스
- `idx_categories_created_at`: 생성일시 기준 정렬용 인덱스

### 4. todo_categories (할일-카테고리 연결 테이블) - V1.1

할일과 카테고리 간의 다대다 관계를 위한 연결 테이블입니다. (V1.1에서 추가)

**필드 구성:**
- `todo_id` (UUID): 할일 ID (todos.id와 연관, 외래키 제약조건 없음)
- `category_id` (UUID): 카테고리 ID (categories.id와 연관, 외래키 제약조건 없음)
- `created_at` (TIMESTAMPTZ): 연결 생성 일시 (UTC 기준)

**기본키:**
- `PRIMARY KEY (todo_id, category_id)`: 할일-카테고리 조합의 복합 기본키 (중복 방지)

**인덱스:**
- `idx_todo_categories_todo_id`: 할일별 카테고리 조회용 인덱스
- `idx_todo_categories_category_id`: 카테고리별 할일 조회용 인덱스
- `idx_todo_categories_created_at`: 생성일시 기준 정렬용 인덱스

## 🔧 Database Constraints & Rules

### Check Constraints

- **todos 테이블**:
  - `chk_todos_status`: 상태 값 제한 (pending, in_progress, completed)
  - `chk_todos_priority`: 우선순위 값 제한 (low, medium, high)
  - `chk_todos_title_length`: 제목 길이 제한 (1-255자)
  - `chk_todos_description_length`: 설명 길이 제한 (최대 1000자)

- **users 테이블**:
  - `chk_users_email_format`: 이메일 형식 검증
  - `chk_users_username_format`: 사용자명 형식 검증 (영문, 숫자, 언더스코어만 허용)

- **categories 테이블**:
  - `chk_categories_color_format`: 색상 코드 형식 검증 (HEX 형식)
  - `chk_categories_name_length`: 이름 길이 제한 (1-100자)

## 📈 Performance Optimization

### Partial Indexes

성능 최적화를 위한 부분 인덱스들:

- `idx_todos_active_user_created`: 완료되지 않은 할일만 대상으로 하는 부분 인덱스
- `idx_todos_with_due_date`: 마감일이 있는 할일만 대상으로 하는 부분 인덱스
- `idx_todos_high_priority`: 높은 우선순위 할일만 대상으로 하는 부분 인덱스

### Statistics & Maintenance

성능 최적화를 위해 주기적으로 통계 정보를 업데이트해야 합니다:
- `ANALYZE users;`
- `ANALYZE todos;`
- `ANALYZE categories;`
- `ANALYZE todo_categories;`

## 📊 Database Statistics Queries

### 사용자별 할일 통계

사용자별 할일 개수 및 완료율을 조회하는 쿼리:

```sql
SELECT
    u.username,
    COUNT(t.id) as total_todos,
    COUNT(CASE WHEN t.status = 'completed' THEN 1 END) as completed_todos,
    COUNT(CASE WHEN t.status = 'pending' THEN 1 END) as pending_todos,
    COUNT(CASE WHEN t.status = 'in_progress' THEN 1 END) as in_progress_todos,
    ROUND(
        COUNT(CASE WHEN t.status = 'completed' THEN 1 END) * 100.0 / NULLIF(COUNT(t.id), 0),
        2
    ) as completion_rate
FROM users u
LEFT JOIN todos t ON u.id = t.user_id
GROUP BY u.id, u.username
ORDER BY total_todos DESC;
```

### 우선순위별 할일 분포

우선순위별 할일 분포를 조회하는 쿼리:

```sql
SELECT
    priority,
    COUNT(*) as todo_count,
    ROUND(COUNT(*) * 100.0 / (SELECT COUNT(*) FROM todos), 2) as percentage
FROM todos
GROUP BY priority
ORDER BY
    CASE priority
        WHEN 'high' THEN 1
        WHEN 'medium' THEN 2
        WHEN 'low' THEN 3
    END;
```

### 마감일 임박 할일 조회

마감일이 임박한 할일을 조회하는 쿼리 (7일 이내):

```sql
SELECT
    t.title,
    t.due_date,
    t.priority,
    t.status,
    u.username,
    EXTRACT(DAY FROM (t.due_date - NOW())) as days_remaining
FROM todos t
JOIN users u ON t.user_id = u.id
WHERE t.due_date IS NOT NULL
    AND t.due_date > NOW()
    AND t.due_date <= NOW() + INTERVAL '7 days'
    AND t.status != 'completed'
ORDER BY t.due_date ASC;
```

## 🔍 Query Examples

### 복합 검색 쿼리

사용자별 할일 검색 (제목/설명, 상태, 우선순위 필터링):

```sql
SELECT
    t.id,
    t.title,
    t.description,
    t.status,
    t.priority,
    t.due_date,
    t.created_at
FROM todos t
WHERE t.user_id = $1
    AND ($2 IS NULL OR t.status = $2)
    AND ($3 IS NULL OR t.priority = $3)
    AND ($4 IS NULL OR (
        to_tsvector('english', t.title) @@ plainto_tsquery('english', $4) OR
        to_tsvector('english', COALESCE(t.description, '')) @@ plainto_tsquery('english', $4)
    ))
ORDER BY
    CASE t.priority
        WHEN 'high' THEN 1
        WHEN 'medium' THEN 2
        WHEN 'low' THEN 3
    END,
    t.created_at DESC
LIMIT $5 OFFSET $6;
```

## 🚀 Migration Scripts

### Version 1.0 → 1.1 (Categories 추가)

V1.1 마이그레이션에서는 카테고리 기능이 추가됩니다:

1. `categories` 테이블 생성
2. `todo_categories` 연결 테이블 생성
3. 관련 인덱스 생성
4. 제약조건 추가

## 📝 Notes

1. **외래키 제약조건 없음**: 성능 최적화와 유연성을 위해 외래키 제약조건을 사용하지 않음
2. **인덱스 전략**: 조인 조건과 자주 사용되는 필터링 조건에만 인덱스 생성
3. **UUID 사용**: 분산 환경에서의 확장성을 고려하여 UUID 사용
4. **TIMESTAMPTZ 사용**: 시간대 정보를 포함한 타임스탬프 사용
5. **전문 검색**: PostgreSQL의 Full-Text Search 기능 활용
6. **부분 인덱스**: 조건부 인덱스로 성능 최적화 및 저장공간 절약

## 📋 Entity Relationships

```
users (1) ----< todos (N)
  |               |
  |               |
  v               v
categories (1) ----< todo_categories >---- todos (N)
  |                                          ^
  |                                          |
  +------------------------------------------+
           (Many-to-Many relationship)
```

### 관계 설명

1. **users ↔ todos**: 1:N 관계
   - 한 사용자는 여러 개의 할일을 가질 수 있음
   - 각 할일은 하나의 사용자에게만 속함

2. **users ↔ categories**: 1:N 관계
   - 한 사용자는 여러 개의 카테고리를 생성할 수 있음
   - 각 카테고리는 하나의 사용자에게만 속함

3. **todos ↔ categories**: N:M 관계
   - 하나의 할일은 여러 카테고리에 속할 수 있음
   - 하나의 카테고리는 여러 할일을 포함할 수 있음
   - `todo_categories` 테이블을 통해 다대다 관계 구현

## 🔄 Data Flow

1. **사용자 등록**: `users` 테이블에 새 사용자 정보 저장
2. **할일 생성**: `todos` 테이블에 새 할일 정보 저장 (user_id 참조)
3. **카테고리 생성**: `categories` 테이블에 새 카테고리 정보 저장 (user_id 참조)
4. **할일-카테고리 연결**: `todo_categories` 테이블에 연결 정보 저장
5. **할일 조회**: 사용자별, 상태별, 카테고리별 필터링을 통한 조회
6. **통계 생성**: 집계 쿼리를 통한 사용자별 완료율 및 분포 통계 생성
