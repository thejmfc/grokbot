# GrokBot

A Discord bot named after the infamous LLM for X, using Ollama as the LLM backend.

## Installation

Ensure you have Rust installed and have cloned the repository.
Run the following command to install dependencies and run the bot in development mode:
```bash
cargo run
```

## Configuration

For now, all configuration is done via environment variables.
A `.env.example` file is provided to show you which variables are required.
After cloning the repository, you should copy `.env.example` to `.env` and populate the file with your own values.

## Packing Helm Chart

To pack the Helm chart, run the following commands:
```bash
cd docs
helm package grokbot
helm repo index . --url https://thejmfc.github.io/grokbot
```