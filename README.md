# PlanITpoker Jira Upload Sanitizer

Little Utility for removing unnecessary stuff from the Jira XML export in order
to prevent upload of potential sensitive data to [PlanITpoker](https://www.planitpoker.com).

# Usage

This application takes a single argument: the XML file exported from JIRA to be sanitized:

```bash
# Run directly from source
$ cargo run -- <file>

# Build it for later use, you will then find the binary in the target directory
$ cargo build [--release]
```

