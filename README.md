# nobg

Remove background from images using AI. Fast (rust btw 🦀), local, and private.

## Features

- **CLI** - Command-line interface for batch processing
- **WebUI** - Browser-based interface (Gradio-style)
- **Multiple models** - Support for various U2Net variants
- **Local processing** - No API calls, runs entirely on your machine
- **Transparency** - Output PNGs with alpha channel

## Installation

To build and install `nobg` run:

```bash
cargo build --release
mv target/release/nobg /usr/local/bin/
```

## Usage

### Model Management

For management `nobg` uses docker-like commands

List available models:

```bash
nobg model ls
```

Download a model:

```bash
nobg model pull silueta
```

Remove a model:

```bash
nobg model rm silueta
```

### CLI

Remove background from an image:

```bash
nobg run silueta input.png output.png
```

### WebUI

Start the web interface:

```bash
nobg webui
```

The web UI will be available at `http://localhost:8080`

Use a custom port:

```bash
nobg webui --port 3000
```

## Showcase

| Input                    | Output                             |
| ------------------------ | ---------------------------------- |
| ![cat](examples/cat.jpg) | ![cat nobg](examples/cat_nobg.png) |

## TODO

- [x] cli

- [x] webui

- [ ] alpha matting

- [x] acceleration
