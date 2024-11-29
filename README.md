# Eyeris - AI Image Analysis Web App

Eyeris is a web application that provides detailed AI-powered image analysis using OpenAI's GPT-4 with vision capabilities. It features a modern, easy-to-use interface for uploading and analyzing images.

## Features

- 🖼️ Simple drag-and-drop image upload interface
- 🤖 Powered by OpenAI's GPT-4 vision model
- 📊 Detailed JSON-structured analysis output
- 🚀 Built with Rust for high performance
- 🌐 Modern web interface

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

2. Set up your OpenAI API key:

```bash
export OPENAI_API_KEY='your-api-key-here'
```

3. Build and run:

```bash
cargo run
```

4. Open your browser and navigate to:

```
http://localhost:3000
```

## Usage

1. Open the web interface in your browser
2. Drag and drop an image or click "Choose File" to select one
3. Wait for the analysis to complete
4. View the detailed JSON analysis of your image

## Tech Stack

- **Backend**: Rust with Axum web framework
- **Frontend**: HTML, CSS, JavaScript
- **AI**: OpenAI GPT-4 Vision API
- **Image Processing**: Rust image crate

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
