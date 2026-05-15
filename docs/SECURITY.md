# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 0.1.x | ✅ |

---

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.**

Send a private report to: **security@substrata.finance** (or open a [GitHub Security Advisory](https://github.com/your-org/Substrata-Contract/security/advisories/new)).

Include:
- A clear description of the vulnerability
- Steps to reproduce or a proof-of-concept
- Potential impact assessment
- Any suggested mitigations

You will receive an acknowledgement within **48 hours** and a resolution timeline within **7 days**.

---

## Scope

The following are in scope:

- `contracts/substrata/src/` — all contract logic
- Authorization bypass (admin, merchant, subscriber)
- Double-charge or re-entrancy vectors
- Token allowance drain beyond approved amount
- Storage manipulation leading to incorrect billing state

The following are **out of scope**:

- Issues in third-party dependencies (report upstream)
- Theoretical attacks with no practical exploit path
- Issues requiring physical access to a key

---

## Disclosure Policy

We follow **coordinated disclosure**:

1. Reporter submits privately.
2. We confirm and investigate within 48 hours.
3. We develop and test a fix.
4. We release the fix and credit the reporter (unless anonymity is requested).
5. Full disclosure published 30 days after the fix is deployed.

---

## Known Limitations (by design)

- The `admin` key is a single address in v0.1. Compromise of this key allows unauthorized billing triggers. A multi-sig upgrade is on the roadmap.
- Subscribers must maintain a sufficient token allowance. The contract cannot enforce allowance renewal.
- Persistent storage entries can expire if TTL is not extended. A keeper bot is recommended for production deployments.
