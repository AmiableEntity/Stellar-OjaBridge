# Contributing to Stellar-OjaBridge


Stellar-OjaBridge aims to provide a modular, high-performance SEP-24 anchor that can be adapted to multiple African markets. By contributing, you're helping expand financial access across the continent.

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Test thoroughly (see Testing Requirements below)
5. Submit a pull request

## Testing Requirements

**CRITICAL**: All code must be tested against the Stellar Testnet before submitting a PR.

- Use Stellar Testnet endpoints (not mainnet)
- Test with Testnet accounts and assets
- Include test results or screenshots in your PR description
- Verify SEP-24 compliance using the Stellar Anchor Validator

## Code Standards

### Rust Backend

- Use idiomatic Rust with proper error handling
- All async functions should use `tokio`
- Follow the existing modular structure
- Add tests for new functionality
- Run `cargo fmt` and `cargo clippy` before committing

### TypeScript Frontend

- Enable strict type checking
- Use functional components with hooks
- Follow Next.js App Router conventions
- Ensure accessibility compliance

## Adding a New Payout Provider

The trait-based architecture makes it easy to add support for new countries:

1. Create a new struct in `backend/src/payouts/`
2. Implement the `PayoutProvider` trait
3. Add configuration for the new provider
4. Write integration tests
5. Document the provider in the README

Example:
```rust
pub struct KenyaPayoutProvider {
    api_key: String,
}

#[async_trait]
impl PayoutProvider for KenyaPayoutProvider {
    async fn send_payout(&self, details: PayoutDetails) -> Result<PayoutResponse, PayoutError> {
        // Implementation
        todo!()
    }

    async fn check_status(&self, transaction_id: &str) -> Result<String, PayoutError> {
        // Implementation
        todo!()
    }

    fn provider_name(&self) -> &str {
        "KenyaProvider"
    }
}
```

## Pull Request Process

1. Update documentation for any new features
2. Add tests covering your changes
3. Ensure all tests pass locally
4. Update the README if needed
5. Reference any related issues

## Questions?

Open an issue or reach out to the maintainers. We're here to help!

---

Built with ❤️ for Drips Wave
