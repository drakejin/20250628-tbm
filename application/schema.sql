-- TODO List App - Database Schema
-- Database: PostgreSQL 15+
-- Character Set: UTF8
-- Collation: utf8_general_ci

-- ============================================================================
-- 1. TABLES
-- ============================================================================

-- 사용자 정보를 저장하는 메인 테이블
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid()
        COMMENT '사용자 고유 식별자 (UUID v4 형식)',

    email VARCHAR(255) NOT NULL UNIQUE
        COMMENT '사용자 이메일 주소 (로그인 ID로 사용, 중복 불가)',

    username VARCHAR(100) NOT NULL UNIQUE
        COMMENT '사용자명 (화면 표시용, 중복 불가, 영문/숫자/언더스코어만 허용)',

    password_hash VARCHAR(255) NOT NULL
        COMMENT '비밀번호 해시값 (bcrypt 알고리즘 사용, cost=12)',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '계정 생성 일시 (UTC 기준)',

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '계정 정보 최종 수정 일시 (UTC 기준)'
);

-- 사용자의 할일 정보를 저장하는 메인 테이블
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid()
        COMMENT '할일 고유 식별자 (UUID v4 형식)',

    user_id UUID NOT NULL
        COMMENT '할일 소유자의 사용자 ID (users.id와 연관, 외래키 제약조건 없음)',

    title VARCHAR(255) NOT NULL
        COMMENT '할일 제목 (필수 입력, 최대 255자, 검색 대상)',

    description TEXT
        COMMENT '할일 상세 설명 (선택 입력, 최대 1000자, 검색 대상)',

    status VARCHAR(20) NOT NULL DEFAULT 'pending'
        COMMENT '할일 진행 상태 (pending: 대기중, in_progress: 진행중, completed: 완료)',

    priority VARCHAR(10) NOT NULL DEFAULT 'medium'
        COMMENT '할일 우선순위 (low: 낮음, medium: 보통, high: 높음)',

    due_date TIMESTAMPTZ
        COMMENT '할일 마감 일시 (UTC 기준, 선택 입력)',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '할일 생성 일시 (UTC 기준)',

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '할일 최종 수정 일시 (UTC 기준)'
);

-- 할일 분류를 위한 카테고리 테이블 (V1.1에서 추가)
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid()
        COMMENT '카테고리 고유 식별자 (UUID v4 형식)',

    user_id UUID NOT NULL
        COMMENT '카테고리 소유자의 사용자 ID (users.id와 연관, 외래키 제약조건 없음)',

    name VARCHAR(100) NOT NULL
        COMMENT '카테고리 이름 (사용자별 중복 불가, 최대 100자)',

    color VARCHAR(7) NOT NULL DEFAULT '#2196F3'
        COMMENT '카테고리 색상 코드 (HEX 형식, 예: #FF5722)',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '카테고리 생성 일시 (UTC 기준)',

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '카테고리 최종 수정 일시 (UTC 기준)'
);

-- 할일과 카테고리 간의 다대다 관계를 위한 연결 테이블 (V1.1에서 추가)
CREATE TABLE todo_categories (
    todo_id UUID NOT NULL
        COMMENT '할일 ID (todos.id와 연관, 외래키 제약조건 없음)',

    category_id UUID NOT NULL
        COMMENT '카테고리 ID (categories.id와 연관, 외래키 제약조건 없음)',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        COMMENT '연결 생성 일시 (UTC 기준)',

    PRIMARY KEY (todo_id, category_id)
        COMMENT '할일-카테고리 조합의 복합 기본키 (중복 방지)'
);

-- ============================================================================
-- 2. INDEXES
-- ============================================================================

-- Users table indexes
-- 이메일 검색용 인덱스 (로그인 시 사용)
CREATE UNIQUE INDEX idx_users_email ON users(email);

-- 사용자명 검색용 인덱스 (중복 체크 및 검색용)
CREATE UNIQUE INDEX idx_users_username ON users(username);

-- 생성일시 기준 정렬용 인덱스 (관리자 페이지에서 사용)
CREATE INDEX idx_users_created_at ON users(created_at DESC);

-- Todos table indexes
-- 사용자별 할일 조회용 인덱스 (가장 자주 사용되는 쿼리)
CREATE INDEX idx_todos_user_id ON todos(user_id);

-- 상태별 할일 조회용 인덱스 (필터링에 사용)
CREATE INDEX idx_todos_status ON todos(status);

-- 우선순위별 할일 조회용 인덱스 (필터링에 사용)
CREATE INDEX idx_todos_priority ON todos(priority);

-- 생성일시 기준 정렬용 인덱스 (최신순 정렬에 사용)
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);

-- 마감일시 기준 정렬용 인덱스 (마감일 임박 순서로 정렬)
CREATE INDEX idx_todos_due_date ON todos(due_date ASC) WHERE due_date IS NOT NULL;

-- 사용자별 상태 조회용 복합 인덱스 (성능 최적화)
CREATE INDEX idx_todos_user_status ON todos(user_id, status);

-- 사용자별 우선순위 조회용 복합 인덱스 (성능 최적화)
CREATE INDEX idx_todos_user_priority ON todos(user_id, priority);

-- 제목 검색용 인덱스 (전문 검색 기능)
CREATE INDEX idx_todos_title_search ON todos USING gin(to_tsvector('english', title));

-- 설명 검색용 인덱스 (전문 검색 기능)
CREATE INDEX idx_todos_description_search ON todos USING gin(to_tsvector('english', description)) WHERE description IS NOT NULL;

-- Categories table indexes
-- 사용자별 카테고리 조회용 인덱스
CREATE INDEX idx_categories_user_id ON categories(user_id);

-- 사용자별 카테고리명 중복 방지용 복합 유니크 인덱스
CREATE UNIQUE INDEX idx_categories_user_name ON categories(user_id, name);

-- 생성일시 기준 정렬용 인덱스
CREATE INDEX idx_categories_created_at ON categories(created_at DESC);

-- Todo_categories table indexes
-- 할일별 카테고리 조회용 인덱스
CREATE INDEX idx_todo_categories_todo_id ON todo_categories(todo_id);

-- 카테고리별 할일 조회용 인덱스
CREATE INDEX idx_todo_categories_category_id ON todo_categories(category_id);

-- 생성일시 기준 정렬용 인덱스
CREATE INDEX idx_todo_categories_created_at ON todo_categories(created_at DESC);

-- ============================================================================
-- 3. CHECK CONSTRAINTS
-- ============================================================================

-- todos 테이블의 상태 값 제한
ALTER TABLE todos ADD CONSTRAINT chk_todos_status
    CHECK (status IN ('pending', 'in_progress', 'completed'));

-- todos 테이블의 우선순위 값 제한
ALTER TABLE todos ADD CONSTRAINT chk_todos_priority
    CHECK (priority IN ('low', 'medium', 'high'));

-- todos 테이블의 제목 길이 제한
ALTER TABLE todos ADD CONSTRAINT chk_todos_title_length
    CHECK (char_length(title) >= 1 AND char_length(title) <= 255);

-- todos 테이블의 설명 길이 제한
ALTER TABLE todos ADD CONSTRAINT chk_todos_description_length
    CHECK (description IS NULL OR char_length(description) <= 1000);

-- users 테이블의 이메일 형식 검증
ALTER TABLE users ADD CONSTRAINT chk_users_email_format
    CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$');

-- users 테이블의 사용자명 형식 검증 (영문, 숫자, 언더스코어만 허용)
ALTER TABLE users ADD CONSTRAINT chk_users_username_format
    CHECK (username ~* '^[A-Za-z0-9_]{3,100}$');

-- categories 테이블의 색상 코드 형식 검증
ALTER TABLE categories ADD CONSTRAINT chk_categories_color_format
    CHECK (color ~* '^#[0-9A-Fa-f]{6}$');

-- categories 테이블의 이름 길이 제한
ALTER TABLE categories ADD CONSTRAINT chk_categories_name_length
    CHECK (char_length(name) >= 1 AND char_length(name) <= 100);

-- ============================================================================
-- 4. PARTIAL INDEXES (Performance Optimization)
-- ============================================================================

-- 완료되지 않은 할일만 대상으로 하는 부분 인덱스 (성능 최적화)
CREATE INDEX idx_todos_active_user_created
    ON todos(user_id, created_at DESC)
    WHERE status != 'completed';

-- 마감일이 있는 할일만 대상으로 하는 부분 인덱스
CREATE INDEX idx_todos_with_due_date
    ON todos(user_id, due_date ASC)
    WHERE due_date IS NOT NULL;

-- 높은 우선순위 할일만 대상으로 하는 부분 인덱스
CREATE INDEX idx_todos_high_priority
    ON todos(user_id, created_at DESC)
    WHERE priority = 'high';

-- ============================================================================
-- 5. STATISTICS & MAINTENANCE
-- ============================================================================

-- 통계 정보 업데이트 (성능 최적화를 위해 주기적 실행 권장)
ANALYZE users;
ANALYZE todos;
ANALYZE categories;
ANALYZE todo_categories;

-- ============================================================================
-- 6. SAMPLE DATA
-- ============================================================================

-- 테스트용 사용자 데이터
INSERT INTO users (id, email, username, password_hash) VALUES
    ('123e4567-e89b-12d3-a456-426614174000', 'john@example.com', 'johndoe', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6ukx.LrUpm'),
    ('223e4567-e89b-12d3-a456-426614174001', 'jane@example.com', 'janedoe', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj6ukx.LrUpm');

-- 테스트용 할일 데이터
INSERT INTO todos (id, user_id, title, description, status, priority, due_date) VALUES
    ('456e7890-e89b-12d3-a456-426614174001', '123e4567-e89b-12d3-a456-426614174000', '장보기', '우유, 빵, 계란 구매', 'pending', 'medium', '2025-07-15 10:00:00+00'),
    ('556e7890-e89b-12d3-a456-426614174002', '123e4567-e89b-12d3-a456-426614174000', '회의 준비', '프레젠테이션 자료 작성', 'in_progress', 'high', '2025-07-09 14:00:00+00');

-- ============================================================================
-- 7. MIGRATION SCRIPTS
-- ============================================================================

-- V1.1 마이그레이션: 카테고리 기능 추가
-- BEGIN;

-- categories 테이블 생성 (이미 위에서 생성됨)
-- CREATE TABLE categories (...);

-- todo_categories 연결 테이블 생성 (이미 위에서 생성됨)
-- CREATE TABLE todo_categories (...);

-- 인덱스 생성 (이미 위에서 생성됨)
-- CREATE INDEX idx_categories_user_id ON categories(user_id);
-- CREATE UNIQUE INDEX idx_categories_user_name ON categories(user_id, name);
-- CREATE INDEX idx_todo_categories_todo_id ON todo_categories(todo_id);
-- CREATE INDEX idx_todo_categories_category_id ON todo_categories(category_id);

-- 제약조건 추가 (이미 위에서 추가됨)
-- ALTER TABLE categories ADD CONSTRAINT chk_categories_color_format
--     CHECK (color ~* '^#[0-9A-Fa-f]{6}$');
-- ALTER TABLE categories ADD CONSTRAINT chk_categories_name_length
--     CHECK (char_length(name) >= 1 AND char_length(name) <= 100);

-- COMMIT;
