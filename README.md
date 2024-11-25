# Image Analysis Service

[![Build Status](https://github.com/yourusername/project-name/workflows/CI/badge.svg)](https://github.com/yourusername/project-name/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/eyeris.svg)](https://crates.io/crates/eyeris)
[![Documentation](https://docs.rs/eyeris/badge.svg)](https://docs.rs/eyeris)

A high-performance image analysis service that leverages AI models to provide detailed analysis of images. Supports multiple AI providers (currently OpenAI and Ollama).

## Features

- 🚀 High-performance image processing with parallel execution
- 🤖 Multiple AI provider support (OpenAI, Ollama)
- 📊 Detailed image analysis with customizable formats
- 🖼️ Automatic image optimization and thumbnail generation
- 📈 Token usage tracking
- 🔒 Rate limiting built-in
- 📝 Comprehensive logging

## Quick Start

```bash
# Clone the repository
git clone https://github.com/scald/eyeris

# Navigate to project directory
cd eyeris

# Configure OpenAI API key (if using OpenAI)
echo "OPENAI_API_KEY=your_key_here" > .env

# Run the service
cargo run
```

## API Usage

Send an image for analysis:

```bash
curl -X POST http://localhost:3000/process \
  -F "image=@path/to/your/image.jpg" \
  -F "provider=ollama" \
  -F "model=moondream" \
  -F "format=json"
```

## Output Formats

- **JSON**: Structured data format for programmatic use
- **Concise**: Brief summary of key elements
- **Detailed**: Comprehensive analysis with multiple aspects
- **List**: Enumerated format for easy reading

## Performance

The service is optimized for performance through:

- Parallel processing with Rayon
- Automatic image optimization
- Efficient memory management
- Rate limiting for stability

## Project Structure

```bash
src/
├── processor.rs # Core processing logic
├── providers/   # AI provider implementations
├── prompts.rs   # Analysis prompt templates
└── main.rs      # Service entry point
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add your changes
4. Write/update tests
5. Submit a pull request

## License

MIT License

---

Built with Rust 🦀
