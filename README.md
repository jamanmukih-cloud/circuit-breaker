# Circuit Breaker 🛡️

Resilience patterns for distributed systems.

## Patterns

| Pattern | Purpose |
|---------|---------|
| Circuit Breaker | Stop cascading failures |
| Bulkhead | Isolate failures |
| Retry | Transient error handling |
| Timeout | Prevent hanging |
| Fallback | Graceful degradation |

## States

```
Closed → (failures > threshold) → Open → (timeout) → Half-Open → (success) → Closed
```

## Quick Start

```rust
let cb = CircuitBreaker::new(5, Duration::from_secs(30));
let result = cb.call(|| external_service.call()).await;
```

## License

MIT