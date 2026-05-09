# Analysis Patterns

Framework-agnostic patterns for codebase analysis and improvement detection.

## Architecture Checks

### Layer Separation
- Handlers/controllers should delegate to services
- Services should not know about HTTP layer
- Database access should be abstracted (repositories/DAOs)
- Types/interfaces should be separate from implementation

### Dependency Direction
- UI → Business Logic → Data Access
- Higher layers depend on lower, never reverse
- Cross-cutting concerns (auth, logging) via middleware/interceptors

### Coupling Indicators
| Warning | Detection |
|---------|-----------|
| God object | Single file with 10+ exports |
| Circular deps | A imports B, B imports A |
| Deep chains | A → B → C → D → E |
| Shotgun surgery | One change touches 10+ files |

## Pattern Checks

### Good Patterns
- Single responsibility per module
- Dependency injection over hard-coded deps
- Configuration externalized
- Error handling centralized

### Anti-Patterns
| Anti-Pattern | Detection |
|--------------|-----------|
| Primitive obsession | Many primitives where object makes sense |
| Magic strings/numbers | Unexplained literals |
| Copy-paste code | Duplicate blocks >10 lines |
| Premature optimization | Complex code without perf need |
| Golden hammer | Same pattern for every problem |

## Performance Checks

### Database
| Issue | Pattern |
|-------|---------|
| N+1 queries | Loop with await/query inside |
| Missing indexes | Slow filters on non-indexed columns |
| Over-fetching | SELECT * when few fields needed |
| Missing pagination | Queries without LIMIT |

### Memory
| Issue | Pattern |
|-------|---------|
| Memory leak | Event listeners not cleaned up |
| Large allocations | Processing entire datasets in memory |
| Unclosed resources | Connections/files not closed |

### Concurrency
| Issue | Pattern |
|-------|---------|
| Blocking I/O | Sync operations in hot path |
| Lock contention | Shared mutable state |
| Race conditions | Async without proper coordination |

## Type Safety Checks

### By Language

| Language | Common Issues |
|----------|--------------|
| TypeScript | `any`, `as unknown`, missing return types |
| Python | Missing type hints, `Any`, `# type: ignore` |
| Go | `interface{}`, unchecked type assertions |
| Java | Raw types, excessive casting |
| Rust | Excessive `unwrap()`, `clone()` abuse |

### Validation Gaps
- API inputs without schema validation
- Environment variables without checks
- User input passed directly to queries
- Missing null/undefined checks

## Security Checks

### Input Validation
- [ ] All user inputs validated and sanitized
- [ ] File uploads have size/type limits
- [ ] URL/path parameters validated
- [ ] JSON parsed with schema

### Authentication & Authorization
- [ ] Protected routes check auth
- [ ] Role-based access implemented
- [ ] Session/token validation on mutations
- [ ] CSRF protection on state-changing ops

### Data Protection
- [ ] No secrets in code
- [ ] PII not logged
- [ ] Error messages don't leak internals
- [ ] Rate limiting on sensitive endpoints

### Common Vulnerabilities
| Issue | Pattern |
|-------|---------|
| SQL injection | String concatenation in queries |
| XSS | Unescaped user content |
| SSRF | User URLs fetched server-side |
| IDOR | Resource IDs not validated for ownership |

## UX/DX Checks

### User Experience
- Loading states for async operations
- Error messages actionable
- Graceful degradation
- Responsive feedback
- Accessible markup

### Developer Experience
- Consistent naming conventions
- Clear file organization
- Documented complex logic
- Type hints/annotations
- Meaningful error messages

## Analysis Output Format

```markdown
## [Category] Improvements

### Critical
| Issue | Location | Impact | Recommendation |
|-------|----------|--------|----------------|
| SQL injection risk | auth.ts:45 | Security breach | Use parameterized queries |

### High Priority
| Issue | Location | Impact | Recommendation |
|-------|----------|--------|----------------|
| N+1 query | users.ts:120 | Performance | Batch fetch or use JOIN |

### Medium Priority
| Issue | Location | Impact | Recommendation |
|-------|----------|--------|----------------|

### Low Priority
| Issue | Location | Impact | Recommendation |
|-------|----------|--------|----------------|
| Missing type | utils.ts:15 | Maintainability | Add type annotation |
```

## Severity Guidelines

| Level | Criteria |
|-------|----------|
| Critical | Security vulnerability, data loss risk |
| High | Performance degradation, bug risk |
| Medium | Maintainability, best practice violation |
| Low | Style, minor improvement |
