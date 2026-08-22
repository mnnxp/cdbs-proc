CADBase Backend Processor
=====

Background service for CADBase that processes file uploads and removes
files marked for deletion from storage.

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `LIMIT_PART` | Number of files processed per cycle | `1` |
| `BUFFER_CAPACITY` | Buffer size (KB) for reading files from S3 | `64` |
| `SLEEPING_TIME` | Sleep duration between cycles (milliseconds) | `200` |

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository and create a feature branch
2. Run `cargo fmt` and `cargo clippy` before committing
3. Open a Pull Request with a clear description of changes
4. Ensure all tests pass: `cargo test`

By contributing, you agree that your contributions will be licensed under the AGPL v3 license.

For major changes, please open an issue first to discuss what you would like to change.

## License

**CADBase** is dual-licensed:

- **AGPL v3** — for open source use, ensuring that all modifications made to the software and offered over a network are shared with the community.
- **Commercial License** — for organizations that wish to use CADBase in proprietary environments or without the source-code disclosure obligations of the AGPL v3.

For commercial licensing inquiries, contact: info@cadbase.rs