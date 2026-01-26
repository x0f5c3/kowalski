# Kowalski Documentation

This directory contains the source for the Kowalski documentation, built with [mdBook](https://rust-lang.github.io/mdBook/).

## Structure

- `src/` - Markdown source files for the documentation
  - `SUMMARY.md` - Table of contents
  - `introduction.md` - Introduction page
  - Other `.md` files - Documentation pages
  - `img/` - Images used in the documentation
- `book.toml` - mdBook configuration
- `book/` - Generated HTML output (excluded from git)

## Building Locally

### Prerequisites

Install mdBook:

```bash
cargo install mdbook
```

### Build the Book

```bash
# Build the book
mdbook build

# Serve the book locally with live reload
mdbook serve --open
```

The book will be available at `http://localhost:3000`.

## Editing Documentation

1. Edit the Markdown files in the `src/` directory
2. Add new pages to `src/SUMMARY.md` to include them in the navigation
3. Run `mdbook serve` to preview changes
4. Commit your changes

## Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the `main` branch. The deployment is handled by the `.github/workflows/deploy-docs.yml` workflow.

## Documentation Guidelines

- Use clear, concise language
- Include code examples where appropriate
- Add images to `src/img/` directory
- Keep the navigation structure in `SUMMARY.md` organized
- Cross-reference related sections using relative links

## Contributing

See the [Contributing Guide](src/contributing.md) for more information on contributing to the documentation.
