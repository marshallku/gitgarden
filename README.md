# Rust HTTP Server Template

This project provides a template for building a scalable HTTP server using Rust. It includes essential components such as an HTTP server, Docker support, and a CI pipeline setup with GitHub Actions.

## Features

- **HTTP Server**: Built using the Axum framework for high performance.
- **Docker & Docker Compose**: Ready-to-use Docker configurations for containerization.
- **CI Pipeline**: GitHub Actions setup for continuous integration and deployment.

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Docker**: Install Docker from [docker.com](https://www.docker.com/).
- **GitHub Actions**: No setup required on your local machine; configuration files are included in the repository.

### Project Structure

- `src/main.rs`: Entry point for the application.
- `src/env/`: Handles environment variable parsing. And handles structure for application state.
- `src/routes/`: Defines application routes.
- `src/utils/`: Utility functions.
- `**/__tests__/`: Tests for the application.
- `Dockerfile`: Docker image definition.
- `docker-compose.yml`: Docker Compose configuration for development and production.
- `.github/workflows/ci.yml`: GitHub Actions configuration for CI pipeline.

### Setup & Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/marshallku/http_server_template.git
   cd http_server_template
   ```

2. **Set up environment variables**:
   Create a `.env` file (or copy `.env.example` to `.env`) at the project root and define the necessary environment variables. For example:

   ```bash
   HOST=127.0.0.1
   PORT=18080
   ```

3. **Run the application locally**:

   ```bash
   cargo run
   ```

   The server will start at the address defined in your environment variables (default: `http://127.0.0.1:18080`).

### Docker & Docker Compose

To build and run the application using Docker:

1. **Build the Docker image**:

   ```bash
   docker build -t rust-http-server-template .
   ```

2. **Run the container**:

   ```bash
   docker run -p 18080:18080 rust-http-server-template
   ```

   Alternatively, use Docker Compose for easier management:

   ```bash
   docker compose up
   ```

### Continuous Integration

The project includes a GitHub Actions workflow for CI. It performs the following steps:

- **Spell Check**: Checks for spelling errors in source code.
- **Build**: Compiles the Rust project.
- **Test**: Runs unit and integration tests.

The workflow is defined in `.github/workflows/ci.yml`. It triggers on every push and pull request to the `master` branch.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue to discuss any changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
