# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for
receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |
| < latest| :x:                |

## Reporting a Vulnerability

Please report (suspected) security vulnerabilities to the project maintainers.
You will receive a response from us within 48 hours. If the issue is confirmed,
we will release a patch as soon as possible depending on complexity.

### How to Report

1. **Do not** open a public GitHub issue for security vulnerabilities
2. Send an email to the project maintainers with:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Any suggested fixes (optional)

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution**: Depends on severity and complexity

### Disclosure Policy

- We will acknowledge receipt of your vulnerability report
- We will confirm the vulnerability and determine its impact
- We will release a fix and publicly disclose the vulnerability
- We will credit you for the discovery (unless you prefer to remain anonymous)

## Security Best Practices

When using tools from this repository:

### dos2unix

- Be cautious when processing files from untrusted sources
- Always backup important files before conversion
- Avoid processing binary files

### gzip

- Verify file integrity after compression/decompression
- Be aware of zip bomb attacks when decompressing untrusted files
- Use appropriate file permissions for compressed files

### htop

- Run with minimal required privileges
- Be aware that process information may contain sensitive data
- Use in trusted environments only

## Security Updates

Security updates will be released as:

1. Patch releases for the affected version
2. Security advisories on GitHub
3. Updates to this document

## Contact

For security-related inquiries, please contact the project maintainers through
GitHub's private vulnerability reporting feature or via email.

---

Thank you for helping keep Build-Your-Own-Utils and its users safe!
