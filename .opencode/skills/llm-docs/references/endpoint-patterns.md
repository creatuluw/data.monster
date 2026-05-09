# Endpoint Detection Patterns

Framework-agnostic patterns for detecting routes/endpoints in codebases.

## JavaScript/TypeScript Frameworks

### SvelteKit
```
src/routes/**/+server.ts      → API endpoint
src/routes/**/+page.server.ts → Page server load
```
Patterns: `export async function GET|POST|PUT|DELETE`, `export const load`, `export const actions`

### Next.js App Router
```
app/**/route.ts    → API endpoint
app/**/page.tsx    → Page component
```
Patterns: `export async function GET|POST`, `NextRequest`, `NextResponse`

### Next.js Pages Router
```
pages/api/**/*.ts  → API endpoint
pages/**/*.tsx     → Page component
```
Patterns: `export default function handler`, `req: NextApiRequest`

### Express.js
```javascript
app.get('/path', handler)
app.post('/path', handler)
router.METHOD('/path', handler)
```

### Fastify
```javascript
fastify.get('/path', options, handler)
fastify.register(routes, { prefix })
```

### NestJS
```typescript
@Controller('path')
@Get(), @Post(), @Put(), @Delete()
```

## Python Frameworks

### FastAPI
```python
@app.get('/path')
@router.post('/path')
async def handler(): ...
```

### Flask
```python
@app.route('/path', methods=['GET'])
def handler(): ...
```

### Django
```
urls.py    → URL patterns
views.py   → View functions/classes
```
Patterns: `path('url', view)`, `class View(APIView)`

### Django REST Framework
```python
class ViewSet(viewsets.ModelViewSet):
    serializer_class = ...
```

## Ruby Frameworks

### Rails
```
config/routes.rb    → Route definitions
app/controllers/    → Controllers
```
Patterns: `resources :name`, `get/post/put/delete`

## Go Frameworks

### net/http
```go
http.HandleFunc('/path', handler)
http.Handle('/path', handler)
```

### Gin
```go
router.GET('/path', handler)
router.POST('/path', handler)
```

### Echo
```go
e.GET('/path', handler)
e.POST('/path', handler)
```

### Fiber
```go
app.Get('/path', handler)
app.Post('/path', handler)
```

## Java/Kotlin Frameworks

### Spring Boot
```java
@RestController
@RequestMapping('/path')
@GetMapping, @PostMapping, @PutMapping, @DeleteMapping
```

### Ktor
```kotlin
routing {
    get('/path') { }
    post('/path') { }
}
```

## PHP Frameworks

### Laravel
```
routes/api.php    → API routes
routes/web.php    → Web routes
```
Patterns: `Route::get()`, `Route::post()`, `Route::resource()`

## Generic Detection

Look for these universal patterns:

| Pattern | Indicates |
|---------|-----------|
| `GET|POST|PUT|DELETE|PATCH` | HTTP methods |
| `/api/`, `/v1/`, `/v2/` | API routes |
| `:id`, `{id}`, `[id]` | Path parameters |
| `request`, `response` | Handler signature |
| `handler`, `controller` | Route handler |
| `router`, `routes` | Routing definition |

## Import Analysis for Dependencies

Track these import patterns:

| Language | Import Pattern |
|----------|---------------|
| JS/TS | `import`, `require` |
| Python | `import`, `from x import` |
| Go | `import` |
| Ruby | `require`, `require_relative` |
| Java | `import` |
| PHP | `use`, `require`, `include` |
