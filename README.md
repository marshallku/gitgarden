# GitGarden

![gitgarden](https://github.com/user-attachments/assets/051c0c51-f257-4163-9109-ec21cdebfc9e)

Git Garden is a creative and engaging visualization tool that transforms your GitHub activity into a beautiful garden. Each element in your garden—whether it's a tree, flower, or decoration—represents different aspects of your contributions on GitHub. Watch your garden grow as you commit to repositories, give stars, and engage with the open-source community. Git Garden also reflects your most used programming language through visual cues in the environment, making your coding journey more interactive and fun!

## Table of Contents

- [Features](#features)
- [Usage](#usage)
- [Self-Hosting](#self-hosting)
- [Local Development](#local-development)
- [Contributing](#contributing)
- [License](#license)

## Features

- Visualizes GitHub contributions as a farm
- Customizable for any GitHub user and specific year
- Dynamically generates SVG images
- Easy to integrate into GitHub profile READMEs

## Usage

To add GitGarden to your GitHub profile, simply insert the following markdown into your README:

```markdown
[![GitGarden](https://gitgarden.marshallku.dev/?user_name=YOUR_GITHUB_USERNAME)](https://github.com/marshallku/gitgarden)
```

Replace `YOUR_GITHUB_USERNAME` with your actual GitHub username.

You can also customize the year by adding the `year` query parameter:

```markdown
[![GitGarden](https://gitgarden.marshallku.dev/?user_name=YOUR_GITHUB_USERNAME&year=2023)](https://github.com/marshallku/gitgarden)
```

## Self-Hosting

GitGarden can be self-hosted using Docker. You have two options:

### Option 1: Using the pre-built image

1. Pull the latest image from GitHub Container Registry:

   ```bash
   docker pull ghcr.io/marshallku/gitgarden:latest
   ```

2. Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

3. Edit the `.env` file with your GitHub token and other necessary configurations.

4. Run the container:

   ```bash
   docker run -p 18080:18080 --env-file .env ghcr.io/marshallku/gitgarden:latest
   ```

### Option 2: Building from source

1. Clone the repository:

   ```bash
   git clone https://github.com/marshallku/gitgarden.git
   cd gitgarden
   ```

2. Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

3. Edit the `.env` file with your GitHub token and other necessary configurations.

4. Build and run the Docker container:

   ```bash
   docker build -t gitgarden .
   docker run -p 18080:18080 gitgarden
   ```

Both options will make GitGarden available at `http://localhost:18080`.

## Local Development

To set up GitGarden for local development:

1. Ensure you have Rust and Cargo installed.
2. Clone the repository and navigate to the project directory.
3. Copy `.env.example` to `.env` and fill in your GitHub token.
4. Run `cargo build` to compile the project.
5. Use `cargo run` to start the server locally.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
