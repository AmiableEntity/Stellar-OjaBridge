# Stellar-OjaBridge

A high-performance Stellar SEP-24 Anchor service enabling seamless fiat-to-crypto on/off-ramps for African markets.

## Features

- **Blazing Fast**: Built with Rust and axum for maximum performance
- **Modular Payouts**: Trait-based adapter system for easy integration with multiple payout providers
- **SEP-24 Compliant**: Full implementation of Stellar's interactive anchor protocol
- **Type-Safe Frontend**: Next.js with TypeScript for robust user interactions
- **Production Ready**: PostgreSQL with proper migrations and error handling

## Architecture

```
stellar-ojabridge/
├── backend/          # Rust axum server (SEP-24 endpoints)
├── frontend/         # Next.js interactive flow UI
├── migrations/       # Database schema migrations
└── docker-compose.yml
```

## Local Setup

### Prerequisites

- Docker & Docker Compose
- Rust (1.70+)
- Node.js (18+)
- PostgreSQL client tools

### Quick Start

1. **Clone and configure**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

2. **Start infrastructure**
   ```bash
   docker-compose up -d
   ```

3. **Run database migrations**
   ```bash
   cd backend
   sqlx migrate run
   ```

4. **Start backend**
   ```bash
   cd backend
   cargo run
   ```

5. **Start frontend**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

6. **Access services**
   - Backend API: http://localhost:8080
   - Frontend: http://localhost:3000
   - PostgreSQL: localhost:5432

## Testing

All contributions should be tested against Stellar Testnet before submission. See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## License

MIT
