# TBM Application

A modern Rust server API built with Axum, following best practices for layered architecture, comprehensive testing, and automatic OpenAPI documentation generation.

## 🚀 Features

- **Modern Architecture**: Clean layered architecture with separation of concerns
- **User Authentication**: Complete user registration and login system
- **Health Check API**: Basic health check endpoint with comprehensive response
- **Database Integration**: PostgreSQL with sqlx and automatic migrations
- **Password Security**: Bcrypt password hashing with proper validation
- **OpenAPI Documentation**: Automatic Swagger UI generation
- **Comprehensive Testing**: Unit and integration tests with mocking
- **Error Handling**: Centralized error handling with proper HTTP status codes
- **Input Validation**: Request validation with detailed error messages
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration management
- **Docker Support**: Ready for containerization

## 📁 Project Structure

```
src/
├── main.rs                # Application entry point
├── lib.rs                 # Library root
├── config/                # Configuration management
├── handlers/              # HTTP handlers (controllers)
├── services/              # Business logic layer
├── repositories/          # Data access layer
├── entities/              # Database models
├── dto/                   # Data Transfer Objects
│   ├── request/           # Request DTOs
│   └── response/          # Response DTOs
├── middleware/            # Custom middleware
├── error/                 # Error handling
└── utils/                 # Utility functions
tests/                     # Integration tests
```

## 🛠️ Technology Stack

- **Framework**: Axum 0.7+
- **Async Runtime**: Tokio 1.32+
- **Database**: PostgreSQL with sqlx 0.7+ (ready for future use)
- **Testing**: tokio-test, mockall
- **Validation**: validator 0.16+
- **Serialization**: serde 1.0+ with serde_json
- **Documentation**: utoipa 4.0+ for OpenAPI generation
- **Error Handling**: thiserror 1.0+
- **Logging**: tracing 0.1+ with tracing-subscriber

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ installed
- Make (optional, for using Makefile commands)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd tbm-application
```

2. Install dependencies:
```bash
make install
# or
cargo fetch
```

3. Build the project:
```bash
make build
# or
cargo build
```

### Running the Application

#### Development Mode
```bash
make run
# or
cargo run
```

#### Development with Auto-reload
```bash
make dev
# or
cargo watch -x run
```

The server will start on `http://localhost:3000` by default.

### Available Endpoints

#### Health Check
- **Health Check**: `GET /health`

#### Authentication
- **User Registration**: `POST /api/v1/auth/register`
- **User Login**: `POST /api/v1/auth/login`
- **User Logout**: `POST /api/v1/auth/logout`

#### Documentation
- **Swagger UI**: `http://localhost:3000/swagger-ui`
- **OpenAPI Spec**: `http://localhost:3000/api-docs/openapi.json`

### Testing

Run all tests:
```bash
make test
# or
cargo test
```

Run tests with verbose output:
```bash
make test-verbose
# or
cargo test -- --nocapture
```

## 📋 Available Make Commands

Run `make help` to see all available commands:

```bash
make help
```

### Key Commands

- `make run` - Run the application
- `make dev` - Run with auto-reload
- `make test` - Run tests
- `make build` - Build the application
- `make quality` - Run all quality checks (format, clippy, test)
- `make health` - Check application health (requires running server)
- `make swagger` - Open Swagger UI (requires running server)

## 🔧 Configuration

The application uses environment variables for configuration:

- `HOST` - Server host (default: `0.0.0.0`)
- `PORT` - Server port (default: `3000`)
- `ENVIRONMENT` - Environment name (default: `development`)
- `LOG_LEVEL` - Log level (default: `info`)
- `DATABASE_URL` - PostgreSQL connection string (required)

### Database Setup

1. Install PostgreSQL and create a database:
```bash
createdb tbm_application
```

2. Set the DATABASE_URL environment variable:
```bash
export DATABASE_URL="postgresql://username:password@localhost/tbm_application"
```

3. The application will automatically run migrations on startup.

### Example Configuration
```bash
HOST=127.0.0.1 \
PORT=8080 \
ENVIRONMENT=production \
LOG_LEVEL=debug \
DATABASE_URL="postgresql://user:pass@localhost/tbm_app" \
make run
```

## 🧪 Testing

The project includes comprehensive testing:

- **Unit Tests**: Located alongside source code
- **Integration Tests**: Located in `tests/` directory
- **Health Check Tests**: Example integration tests for the health endpoint

### Running Specific Tests

```bash
# Run only health tests
cargo test health

# Run with output
cargo test -- --nocapture
```

## 📚 API Documentation

The API documentation is automatically generated using OpenAPI/Swagger:

1. Start the server: `make run`
2. Open Swagger UI: `make swagger` or visit `http://localhost:3000/swagger-ui`

### API Usage Examples

#### Health Check
```bash
curl -X GET http://localhost:3000/health
```

Response:
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0",
  "environment": "development"
}
```

#### User Registration
```bash
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "username": "johndoe",
    "password": "securepassword123"
  }'
```

Response (201):
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "user@example.com",
  "username": "johndoe",
  "created_at": "2025-07-09T12:00:00Z"
}
```

#### User Login
```bash
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securepassword123"
  }'
```

Response (200):
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

#### User Logout
```bash
curl -X POST http://localhost:3000/api/v1/auth/logout \
  -H "Authorization: Bearer {token}"
```

Response (204): No Content

## 🐳 Docker Support

Build and run with Docker:

```bash
# Build Docker image
make docker-build

# Run Docker container
make docker-run

# Or use docker-compose
make docker-compose-up
```

## 🔍 Code Quality

The project enforces high code quality standards:

```bash
# Format code
make fmt

# Run linter
make clippy

# Run all quality checks
make quality
```

## 🚀 Development Workflow

1. **Setup**: `make dev-setup`
2. **Development**: `make dev` (auto-reload)
3. **Testing**: `make test`
4. **Quality Check**: `make quality`
5. **Build**: `make build`

## 📈 Future Enhancements

The project structure is ready for:

- Database integration (PostgreSQL with sqlx)
- User authentication and authorization
- Additional API endpoints
- Middleware implementation
- Advanced error handling
- Metrics and monitoring
- CI/CD pipeline integration

## 🤝 Contributing

1. Follow the existing code structure and patterns
2. Write tests for new functionality
3. Run `make quality` before submitting
4. Update documentation as needed

## 📄 License

This project is licensed under the MIT License.
