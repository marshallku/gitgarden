
# Git Garden 🌱

Git Garden is a creative and engaging visualization tool that transforms your GitHub activity into a beautiful garden. Each element in your garden—whether it's a tree, flower, or decoration—represents different aspects of your contributions on GitHub. Watch your garden grow as you commit to repositories, give stars, and engage with the open-source community. Git Garden also reflects your most used programming language through visual cues in the environment, making your coding journey more interactive and fun!

## Table of Contents

- [Features](#features)
- [How It Works](#how-it-works)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Interactive Garden**: Your GitHub activity is represented by a dynamic garden where trees, flowers, and other elements grow and change.
- **Gamification**: Earn trees by giving stars to repositories, and grow flowers by committing to your projects.
- **Real-Time Updates**: Your garden evolves in real-time based on your GitHub activity.
- **Language Visualization (planned)**: The most used programming language is visually reflected in the garden through color schemes, decorations, and special effects.
- **Customizable Themes (planned)**: Different themes are available based on the programming languages you use the most.

## How It Works

1. **GitHub Integration**: Git Garden connects to your GitHub account to track your contributions, repositories, stars, and more.
2. **Visualization**: Based on your activities, the garden elements grow and change. Each element (like a tree or flower) represents a different kind of activity.
3. **Language Representation (planned)**: The most used programming language in your repositories influences the appearance of your garden—whether through colors, shapes, or seasonal themes.

### Example

- **Commit More, Grow More**: Every time you commit to a project, a new flower blooms in your garden.
- **Star Repositories, Grow Trees**: Give stars to other repositories, and watch trees sprout up in your garden.
- **Language Influence (planned)**: If JavaScript is your most used language, the garden might show yellow flowers, while Python could bring a green theme.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [GitHub API Token](https://github.com/settings/tokens) for accessing GitHub data

### Clone the Repository

```bash
git clone https://github.com/your-username/git-garden.git
cd git-garden
```

### Set Up Environment Variables

Create a `.env` file in the root of your project and add your GitHub API token:

```bash
GITHUB_TOKEN=your_github_api_token_here
```

### Run the Application

```bash
cargo run
# or
docker compose up
```

## Usage

1. **Connect Your GitHub Account**: Log in with your GitHub account to start visualizing your activity.
2. **Watch Your Garden Grow**: As you code, commit, and contribute, return to your garden to see how it evolves.
3. **Explore Different Themes (planned)**: Use the settings to explore different themes based on your most used programming languages.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feat/your-feature-name`).
3. Commit your changes (`git commit -m 'Add your feature'`).
4. Push to the branch (`git push origin feat/your-feature-name`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
