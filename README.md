# Eyeris - AI Image Analysis Service

Eyeris is a powerful image analysis service that provides both a web interface and REST API for AI-powered image analysis using OpenAI's GPT-4 vision capabilities.

## Features

- 🖼️ Simple drag-and-drop web interface
- 🔌 RESTful API for integration
- 🤖 Powered by OpenAI's GPT-4 vision model
- 📊 Detailed JSON-structured analysis output
- 🚀 Built with Rust for high performance
- 🌐 CORS-enabled for web applications

## Getting Started

### Prerequisites

- Rust (latest stable version)
- An OpenAI API key

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/eyeris.git
cd eyeris
```

1. Set up your OpenAI API key:

```bash
export OPENAI_API_KEY='your-api-key-here'
```

1. Build and run:

```bash
cargo run
```

1. Access the service:

- Web Interface: `http://localhost:3000`
- API Endpoint: `http://localhost:3000/api/v1`
- API Documentation: `http://localhost:3000/docs`

## Usage

### Web Interface

1. Open the web interface in your browser
2. Drag and drop an image or click "Choose File"
3. Wait for the analysis to complete
4. View the detailed JSON analysis

### API

```bash
# Analyze an image
curl -X POST http://localhost:3000/api/v1/analyze \
  -F "image=@path/to/your/image.jpg"

# Health check
curl http://localhost:3000/api/v1/health
```

See the [API Documentation](docs/api.md) for detailed endpoint information and examples.

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: HTML, CSS, JavaScript
- **AI**: OpenAI GPT-4 Vision API
- **Image Processing**: Rust image crate

## API Documentation

The API provides endpoints for image analysis and health checking. All responses are in JSON format.

### Key Endpoints

- `POST /api/v1/analyze` - Analyze an image
- `GET /api/v1/health` - Check service health

For detailed API documentation, examples, and integration guides, see the [API Documentation](docs/api.md).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
