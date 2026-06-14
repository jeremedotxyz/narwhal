cat <<EOF > ~/Desktop/narwhal/README.md
# Narwhal LLM Harness

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

**Narwhal** is a high-performance, asynchronous LLM (Large Language Model) evaluation harness built in Rust. It is designed to provide a lightweight yet robust framework for benchmarking model responses, testing prompt variations, and generating structured evaluation reports.

## 🚀 Features

- **Asynchronous Execution:** Built on the \`tokio\` runtime for non-blocking API interactions.
- **Strict Type Safety:** Leverages Rust's ownership and type system to ensure data integrity.
- **Configurable Endpoints:** Compatible with OpenAI, Anthropic, and local LLM servers (like Ollama or LocalAI).
- **Automated Reporting:** Generates a structured \`results.json\` and a formal evaluation report.

## 📁 Project Structure

\`\`\`text
narwhal/
├── src/
│   └── main.rs          # Core logic and API client
├── Cargo.toml           # Dependency management
├── evaluation_report.md # Narrative evaluation findings
├── README.md            # Project documentation
└── results.json         # Output generated after execution (post-run)
\`\`\`

## 🛠 Installation & Setup

### Prerequisites
- **Rust Toolchain:** Install via [rustup.rs](https://rustup.rs/)
- **API Key:** An active API key from an LLM provider.

### Building from Source
1. Navigate to the project directory:
   \`\`\`bash
   cd ~/Desktop/narwhal
   \`\`\`
2. Compile the project:
   \`\`\`bash
   cargo build --release
   \`\`\`

## ⚙️ Configuration

Open \`src/main.rs\` and update the following variables in the \`main\` function:

- **api_key:** Your provider's API key.
- **endpoint:** The URL of the completions API.
- **model:** The specific model ID (e.g., \`gpt-4\`, \`claude-3-opus\`).

## 📋 Usage

To execute the evaluation harness, run:

\`\`\`bash
cargo run
\`\`\`

The harness will:
1. Load the predefined prompt set.
2. Send requests concurrently to the configured endpoint.
3. Capture responses and timestamps.
4. Save the full dataset to \`results.json\`.

## 📊 Evaluation Report

The project includes an \`evaluation_report.md\` file. This document provides a qualitative analysis of the harness's architecture, memory safety profiles, and recommendations for scaling evaluation datasets.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
EOF

echo "Detailed README.md has been created in your narwhal folder!"
