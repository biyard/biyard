# Biyard Console

A Firebase-style console for managing Biyard platform accounts, credentials, and settings.

## Features

### 🔐 Authentication
- **Sign Up**: Create new account with email, password, and name
- **Sign In**: Authenticate with existing credentials
- **Session Management**: Automatic session handling with HTTP-only cookies
- **Account Withdrawal**: Permanently delete account with confirmation dialog

### 🌍 Internationalization (i18n)
- **Multi-language Support**: English and Korean translations
- **Language Switcher**: Easy toggle between languages
- **Persistent Settings**: Language preference saved to localStorage

### 🎨 Theme Support
- **Light/Dark Mode**: Toggle between light and dark themes
- **Persistent Theme**: Theme preference saved to localStorage
- **System Integration**: Automatic dark mode class application

### 🔑 API Credential Management
- **Create Credentials**: Generate new API keys for service integration
- **View Keys**: List all active API credentials
- **Copy to Clipboard**: One-click copy for API keys
- **Key Masking**: Secure display of API keys

### ⚙️ Account Settings
- **Profile View**: Display account information (name, email, ID)
- **Account Details**: Creation date and account metadata
- **Danger Zone**: Secure account deletion with confirmation

## Project Structure

```
web/src/
├── components/          # Shared components
│   └── ProtectedRoute.tsx
├── contexts/           # React contexts
│   ├── AuthContext.tsx    # Authentication state
│   └── ThemeContext.tsx   # Theme management
├── features/           # Feature-based modules
│   ├── auth/
│   │   ├── api/           # Auth API hooks
│   │   │   ├── use-signin.ts
│   │   │   ├── use-signup.ts
│   │   │   └── use-withdrawal.ts
│   │   └── components/    # Auth pages
│   │       ├── SignInPage.tsx
│   │       └── SignUpPage.tsx
│   ├── dashboard/
│   │   └── components/
│   │       └── DashboardPage.tsx
│   ├── settings/
│   │   └── components/
│   │       └── SettingsPage.tsx
│   └── credentials/
│       └── components/
│           └── CredentialsPage.tsx
├── i18n/               # Internationalization
│   ├── config.ts
│   └── locales/
│       ├── en.json
│       └── ko.json
├── lib/                # Utilities
│   └── api-client.ts   # HTTP client with session support
├── types/              # TypeScript types
│   └── account.ts
├── App.tsx             # Main app with routing
└── main.tsx            # Entry point
```

## Backend API Integration

The console integrates with Biyard's v1 DynamoDB-backed APIs:

### Endpoints Used

| Endpoint | Method | Description | Auth Required |
|----------|--------|-------------|---------------|
| `/v1/accounts/signup` | POST | Create new account | No |
| `/v1/accounts/signin` | POST | Sign in to account | No |
| `/v1/accounts/withdrawal` | POST | Delete account | Yes |

### Request/Response Types

**Sign Up Request:**
```typescript
{
  name: string;
  email: string;
  hashed_password: string; // Plain password (backend hashes it)
}
```

**Sign In Request:**
```typescript
{
  email: string;
  password: string;
}
```

**Account Response:**
```typescript
{
  pk: string;          // Account UUID
  name: string;
  email: string;
  created_at: number;  // Unix timestamp
}
```

### Session Management
- Sessions are managed via HTTP-only cookies
- Cookie automatically set on successful signup/signin
- All API requests include `credentials: 'include'` for cookie transmission
- Protected routes redirect to signin if session is invalid

## Development

### Prerequisites
- Node.js 18+
- pnpm 10.18.2+
- Backend API running (default: `http://localhost:8080`)

### Setup

1. **Install dependencies:**
   ```bash
   cd web
   pnpm install
   ```

2. **Configure environment:**
   ```bash
   cp .env.example .env
   # Edit .env to set VITE_API_BASE_URL if needed
   ```

3. **Run development server:**
   ```bash
   pnpm dev
   ```
   Access at `http://localhost:5173`

### Build for Production

```bash
pnpm build
```

Output: `web/dist/`

### Preview Production Build

```bash
pnpm preview
```

## Technology Stack

### Core
- **React 19.1.1**: UI framework
- **TypeScript 5.9.3**: Type safety
- **Vite 7.1.7**: Build tool and dev server

### Routing & State
- **React Router DOM 7.9.4**: Client-side routing
- **TanStack Query 5.90.5**: Server state management

### Styling
- **Tailwind CSS 4.1.16**: Utility-first CSS
- **Lucide React 0.546.0**: Icon library

### Internationalization
- **i18next 25.6.0**: i18n framework
- **react-i18next 16.1.6**: React bindings

## Usage Guide

### Sign Up Flow
1. Navigate to `/signup`
2. Enter name, email, password, and confirm password
3. Submit form
4. Automatically redirected to dashboard on success
5. Session cookie stored for authentication

### Sign In Flow
1. Navigate to `/signin`
2. Enter email and password
3. Submit form
4. Redirected to dashboard on success

### Managing API Credentials
1. Navigate to `/credentials` from dashboard
2. Click "Create New Credential"
3. Enter credential name
4. Generated API key displayed (copy immediately)
5. View all credentials in table

### Account Withdrawal
1. Navigate to `/settings` from dashboard
2. Scroll to "Delete Account" section
3. Click "Delete Account" button
4. Confirm in dialog
5. Account permanently deleted
6. Redirected to sign in page

### Theme & Language
- **Theme Toggle**: Click sun/moon icon in header
- **Language Toggle**: Click globe icon in header

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VITE_API_BASE_URL` | `http://localhost:8080` | Backend API base URL |

## Security Considerations

### Session Security
- HTTP-only cookies prevent XSS attacks
- Sessions managed by backend (DynamoDB session store)
- Credentials sent with every request via `credentials: 'include'`

### Password Handling
- Passwords transmitted over HTTPS in production
- Backend hashes passwords using BCrypt
- No password stored in frontend state

### API Key Display
- Keys masked by default (showing first 12 and last 4 chars)
- Full key shown only once on creation
- Copy to clipboard for secure storage

## Future Enhancements

### Backend Integration Needed
- Implement actual API credential CRUD endpoints (`/v2/credentials/*`)
- Add API key validation and usage tracking
- Implement refresh token mechanism
- Add 2FA support

### Frontend Improvements
- Add form validation library (e.g., React Hook Form + Zod)
- Implement toast notifications for user feedback
- Add loading skeletons for better UX
- Create onboarding flow for new users
- Add project management features
- Implement API usage dashboard

## Troubleshooting

### Build Issues
```bash
# Clear node_modules and reinstall
rm -rf node_modules pnpm-lock.yaml
pnpm install

# Clear Vite cache
rm -rf node_modules/.vite
pnpm build
```

### CORS Errors
Ensure backend allows requests from frontend origin:
```rust
// In Rust backend
.layer(
    CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
)
```

### Session Not Persisting
- Check that backend sets `SameSite=Lax` or `None` (with Secure in production)
- Ensure `credentials: 'include'` in fetch requests
- Verify backend session store is working

## Contributing

1. Follow feature-driven development pattern
2. Place related files in `features/<feature-name>/`
3. Use TypeScript strict mode
4. Add translations to both `en.json` and `ko.json`
5. Ensure dark mode compatibility

## License

Part of the Biyard platform.
