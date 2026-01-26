# Biyard CDK Deployment Plan

## Overview

CloudFront + ALB + Axum 구조로 SSR과 정적 파일 배포를 분리합니다.

## Domain Structure

```
1. api.dev.biyard.co       → ALB 직접 (기존 유지, CloudFront 거치지 않음)
2. dev.biyard.co           → CloudFront → ALB (Axum /landing 경로)
3. console.dev.biyard.co   → CloudFront → ALB (Axum /console 경로)
```

## Traffic Flow

### Static Files (S3 CDN)

```
dev.biyard.co/landing/app.js          → CloudFront → S3 (cached)
dev.biyard.co/landing/styles.css      → CloudFront → S3 (cached)
dev.biyard.co/landing/favicon.ico     → CloudFront → S3 (cached)
console.dev.biyard.co/console/app.js  → CloudFront → S3 (cached)
```

### Page Requests (Axum SSR)

```
dev.biyard.co/                → CloudFront → ALB → Axum /landing (SSR)
dev.biyard.co/login           → CloudFront → ALB → Axum /landing/login (SSR)
dev.biyard.co/signup          → CloudFront → ALB → Axum /landing/signup (SSR)
dev.biyard.co/landing         → CloudFront → ALB → Axum /landing/landing (SSR)

console.dev.biyard.co/        → CloudFront → ALB → Axum /console (SSR)
console.dev.biyard.co/dashboard → CloudFront → ALB → Axum /console/dashboard (SSR)
```

### API Requests

```
api.dev.biyard.co/v1/projects → ALB 직접 (latency-based routing)
api.dev.biyard.co/m1/admin    → ALB 직접
```

## CloudFront Configuration

### Origins

1. **ALB Origin**: Axum backend for SSR
2. **S3 Origin**: Static assets (JS, CSS, images, etc.)

### Behaviors Priority

```
Priority 1: /landing/* → S3 (cached static files)
Priority 2: /console/* → S3 (cached static files)
Priority 3: /*.js      → S3 (cached)
Priority 4: /*.css     → S3 (cached)
Priority 5: /*.ico     → S3 (cached)
... (other static file extensions)
Default:    /*         → ALB (no cache, with CloudFront Function)
```

**Note**: `/landing/*` and `/console/*` behaviors have higher priority than default behavior, so:
- `dev.biyard.co/landing/app.js` → S3 (matches `/landing/*`)
- `dev.biyard.co/landing` → ALB (no file extension, goes to defaultBehavior, Function adds `/landing` prefix → `/landing/landing`)

### CloudFront Function (Domain Routing)

도메인별로 경로를 자동 변환:

```javascript
function handler(event) {
  var request = event.request;
  var host = request.headers.host.value;
  var uri = request.uri;

  // 정적 파일은 그대로 통과
  var hasFileExtension = /\\.[a-zA-Z0-9]+$/.test(uri);
  if (hasFileExtension) {
    return request;
  }

  // 이미 /landing 또는 /console로 시작하면 그대로
  if (uri.startsWith('/landing') || uri.startsWith('/console')) {
    return request;
  }

  // console 도메인이면 /console 추가
  if (host.startsWith('console.')) {
    request.uri = '/console' + uri;
  }
  // 그 외 (landing 도메인)는 /landing 추가
  else {
    request.uri = '/landing' + uri;
  }

  return request;
}
```

## Axum Router Structure

```rust
// api/src/api_main.rs
pub async fn api_main() -> Result<Router, crate::Error> {
    let app = by_axum::new();

    let api_router = route().await?;

    let app = app
        .nest("/v1", v1::route()?)
        .nest("/m1", m1::route()?)
        .nest("/landing", landing::route()?)  // /landing/* 경로
        .nest("/console", console::route()?)  // /console/* 경로
        .merge(api_router);

    Ok(app)
}
```

```rust
// api/src/controllers/landing/mod.rs
pub fn route() -> Router {
    Router::new()
        .route("/", get(index))           // /landing/
        .route("/login", get(index))      // /landing/login
        .route("/signup", get(index))     // /landing/signup
        .fallback(get(index))             // /landing/* (SPA fallback)
}
```

```rust
// api/src/controllers/console/mod.rs
pub fn route() -> Router {
    Router::new()
        .route("/", get(index))           // /console/
        .route("/dashboard", get(index))  // /console/dashboard
        .fallback(get(index))             // /console/* (SPA fallback)
}
```

## Request Flow Examples

### Example 1: Landing Page
```
User: dev.biyard.co/
    ↓
CloudFront (defaultBehavior)
    ↓
CloudFront Function: "/" → "/landing/"
    ↓
ALB → Axum /landing/ → landing::index (Askama SSR)
    ↓
HTML with <script src="/landing/app.js">
    ↓
Browser requests /landing/app.js
    ↓
CloudFront (/landing/* behavior) → S3 (cached)
```

### Example 2: Login Page
```
User: dev.biyard.co/login
    ↓
CloudFront (defaultBehavior)
    ↓
CloudFront Function: "/login" → "/landing/login"
    ↓
ALB → Axum /landing/login → landing::index (Askama SSR)
```

### Example 3: Console Dashboard
```
User: console.dev.biyard.co/dashboard
    ↓
CloudFront (defaultBehavior)
    ↓
CloudFront Function: "/dashboard" → "/console/dashboard"
    ↓
ALB → Axum /console/dashboard → console::index (Askama SSR)
```

### Example 4: Static File
```
User: dev.biyard.co/landing/app.js
    ↓
CloudFront (/landing/* behavior) → S3 (cached)
(CloudFront Function does NOT execute for additionalBehaviors)
```

## Local Development

### Setup

```bash
# /etc/hosts
127.0.0.1 dev.biyard.local
127.0.0.1 console.dev.biyard.local
127.0.0.1 api.dev.biyard.local
```

### Testing

```bash
# Landing page
curl http://dev.biyard.local:3000/landing/
curl http://dev.biyard.local:3000/landing/login

# Console page
curl http://console.dev.biyard.local:3000/console/
curl http://console.dev.biyard.local:3000/console/dashboard

# API
curl http://api.dev.biyard.local:3000/v1/projects
```

## Deployment Steps

1. **Deploy RegionalClusterStack** (ALB + ECS)
   ```bash
   cd cdk
   cdk deploy biyard-dev-cluster
   ```

2. **Build and upload static assets to S3**
   ```bash
   cd web
   pnpm build
   aws s3 sync dist/landing s3://BUCKET-NAME/landing/ --delete
   aws s3 sync dist/console s3://BUCKET-NAME/console/ --delete
   ```

3. **Deploy GlobalAccelStack** (CloudFront)
   ```bash
   cd cdk
   cdk deploy GlobalAccel
   cdk deploy Console
   ```

## Testing Checklist

- [ ] `dev.biyard.co/` returns Landing HTML (Askama SSR)
- [ ] `dev.biyard.co/login` returns Landing HTML (Askama SSR)
- [ ] `dev.biyard.co/landing/app.js` returns JS from S3 (cached)
- [ ] `console.dev.biyard.co/` returns Console HTML (Askama SSR)
- [ ] `console.dev.biyard.co/dashboard` returns Console HTML (Askama SSR)
- [ ] `console.dev.biyard.co/console/app.js` returns JS from S3 (cached)
- [ ] `api.dev.biyard.co/v1/projects` returns JSON from Axum (no CloudFront)
- [ ] CloudFront cache headers: `X-Cache: Hit from cloudfront` for static files
- [ ] CloudFront cache headers: `X-Cache: Miss from cloudfront` for SSR pages
- [ ] Session cookies work across dev.biyard.co and console.dev.biyard.co

## Security Considerations

1. **ALB Access**: Keep ALB public but verify `Host` header in Axum
2. **CORS**: Configure CORS to allow CloudFront origins
3. **Cookies**: Set cookies with `Domain=.biyard.co` for cross-subdomain sharing
4. **HTTPS**: All traffic uses HTTPS (CloudFront enforces REDIRECT_TO_HTTPS)

## Cost Estimation (Monthly, 100K visitors)

- CloudFront: ~$200 (2 distributions)
- ALB: ~$20
- S3: ~$5
- ECS Fargate: ~$50 (2 tasks)
- **Total**: ~$275/month
