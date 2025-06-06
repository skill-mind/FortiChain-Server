# FortiChain Server Logging System

## Overview
The FortiChain Server implements a comprehensive Unit of Work (UoW) logging system that tracks the complete lifecycle of each HTTP request. This system provides end-to-end observability, making it easier to debug issues, monitor performance, and maintain audit trails.

## Features
- Unique Request ID tracking
- Request/Response lifecycle logging
- Error tracking with full context
- Sensitive data redaction
- Performance metrics (request duration)
- Environment-aware error handling

## Log Format
Each log entry includes:
- Timestamp
- Request ID
- Log level
- Message
- Context data (varies by log type)

### Request Start Log
```json
{
  "timestamp": "2024-03-14 10:15:30",
  "requestId": "550e8400-e29b-41d4-a716-446655440000",
  "level": "info",
  "message": "Request started",
  "body": "...",
  "query": "...",
  "params": "...",
  "headers": {
    "authorization": "[REDACTED]",
    ...
  }
}
```

### Request Completion Log
```json
{
  "timestamp": "2024-03-14 10:15:31",
  "requestId": "550e8400-e29b-41d4-a716-446655440000",
  "level": "info",
  "message": "Request completed",
  "statusCode": 200,
  "duration": 1000,
  "responseBody": "..."
}
```

### Error Log
```json
{
  "timestamp": "2024-03-14 10:15:31",
  "requestId": "550e8400-e29b-41d4-a716-446655440000",
  "level": "error",
  "message": "Request failed",
  "error": {
    "message": "Error message",
    "stack": "Error stack trace",
    "statusCode": 500,
    "status": "error",
    "isOperational": true
  }
}
```

## Configuration
The logging system can be configured using environment variables:

- `LOG_LEVEL`: Set the minimum log level (default: 'info')
- `NODE_ENV`: Set the environment (development/production)

## Accessing Logs
Logs are written to two files in the `logs` directory:
- `combined.log`: Contains all logs
- `error.log`: Contains only error logs

## Tracing Requests
To trace a request:
1. Get the Request ID from the `X-Request-ID` response header
2. Search the logs for this ID to see the complete request lifecycle
3. All related logs (start, completion, errors) will be linked by this ID

## Security
- Sensitive data (passwords, tokens, etc.) is automatically redacted
- Error details are only shown in development environment
- Request IDs are UUID v4 for uniqueness and security

## Performance
- Uses AsyncLocalStorage for minimal overhead
- Efficient log format
- Automatic context cleanup

## Error Handling
The system provides two types of errors:
1. Operational Errors: Expected errors (e.g., validation failures)
2. Programming Errors: Unexpected errors (e.g., runtime exceptions)

In production:
- Operational errors return detailed messages
- Programming errors return generic messages
- All errors are logged with full context

## Testing
Run the logging system tests:
```bash
npm test tests/unit/logging.test.js
```

## Best Practices
1. Always use the provided `AppError` class for operational errors
2. Include relevant context in error messages
3. Use appropriate log levels (debug, info, error)
4. Don't log sensitive information
5. Use request IDs for tracing issues 