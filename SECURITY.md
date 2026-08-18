# Security

slotbench measures **public** RPC / gRPC endpoints. It should not hold chain keys.

## Rules

- API keys for paid endpoints live in `.env` (`chmod 0600`), never in git, never in the public board HTML.
- Do not log full keys. Redact in any debug dump.
- Measurement hosts must not sign transactions.
- Ranking can annoy vendors. Keep the method public and give a dispute path. Do not fabricate numbers.

## Reporting

Open a private GitHub security advisory if a leaked key or a measurement integrity issue appears.
