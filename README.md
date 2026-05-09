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
nobg ls
```

Download one or more models:

```bash
nobg pull silueta
nobg pull model1 model2
```

Remove one or more models:

```bash
nobg rm silueta
nobg rm model1 model2
```

Prune all downloaded models:

```bash
nobg prune -y
```

### CLI

Remove background from an image:

```bash
nobg run silueta input.png output.png
```

Output to a prefixed filename (omitting output):

```bash
nobg run silueta input.png
# Saves as: input_nobg.png
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
