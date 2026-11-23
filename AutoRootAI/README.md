# AutoRootAI 

A Rust-based autonomous Linux privilege-escalation engine. Runs recon checks (SUID, sudo, groups, kernel, creds) on a target, sends the results to an LLM (Groq) framed as an authorized pentest engagement, and gets back a structured exploit recommendation:

SEVERITY: NONE/LOW/MED/HIGH
FINDING: <description>
ACTION: <single non-interactive command chain>

### Features
- Static musl build that runs on any target without glibc version issues
- Routes AI calls through a SOCKS5 tunnel (chisel) when the target has no internet access, with a local no-proxy mode for development
- Deterministic output (temperature 0) framed as an authorized-engagement prompt, reducing model refusals on sensitive content
- Local integration tests (`cargo test`) validate real AI output against known vulnerable and clean inputs — no target or tunnel required to test logic changes

### Known limitations
- The free-tier model occasionally fails to conclude on ambiguous/edge-case input (long internal reasoning loop, no final answer within token budget), fix planned via API key rotation/retry
- `.env` loading currently requires a local `.env` copy alongside the binary
