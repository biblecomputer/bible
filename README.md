# Bible Study App

A modern, fast, and responsive Bible study application built with Rust and Leptos. Features fuzzy search, chapter navigation, and a clean, distraction-free interface for studying scripture.

## Development

### Prerequisites

This project uses [Nix](https://nixos.org/) for reproducible development environments. You'll need to:

1. [Install Nix](https://nixos.org/download.html)
2. [Enable Nix Flakes](https://nixos.wiki/wiki/Flakes#Enable_flakes) (add `experimental-features = nix-command flakes` to your Nix configuration)

### Running the Development Server

Start the development server with hot reload:

```bash
nix run .#dev
```

This will:
- Generate Tailwind CSS assets
- Start the Tailwind CSS watcher for style changes
- Launch the Trunk development server
- Open your browser automatically at `http://localhost:8080`

The server will automatically reload when you make changes to Rust code or styles.

### Building for Production

Build the optimized static site:

```bash
nix build
```

The built assets will be available in `./result/` and can be served by any static web server.

### Development Shell

Enter a development shell with all tools available:

```bash
nix develop
```

This gives you access to:
- Rust toolchain with WASM target
- Trunk (WASM web application bundler)
- Tailwind CSS
- All project dependencies

## Contributing

We welcome contributions! Here's how you can help:

### Getting Started

1. Fork this repository
2. Clone your fork: `git clone https://github.com/yourusername/bible.git`
3. Enter the development environment: `nix develop`
4. Make your changes
5. Test your changes: `nix run .#dev`
6. Submit a pull request

### Areas for Contribution

- 🐛 **Bug Fixes** - Help improve stability and user experience
- ✨ **Features** - Add new functionality like bookmarks, notes, or themes
- 🎨 **UI/UX** - Improve the interface and user experience
- 📚 **Documentation** - Help improve docs and examples
- 🧪 **Testing** - Add tests for better code quality
- 🌍 **Translations** - Add support for multiple languages

### Code Style

- Follow Rust conventions and use `cargo fmt`
- Keep components small and focused
- Write clear commit messages
- Add tests for new functionality

### Reporting Issues

Found a bug or have a feature request? Please [open an issue](https://github.com/yourusername/bible/issues) with:

- Clear description of the problem or feature
- Steps to reproduce (for bugs)
- Your environment (OS, browser)
- Screenshots if applicable

## Technology Stack

- **Frontend**: [Leptos](https://leptos.dev/) - Rust web framework
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS
- **Build Tool**: [Trunk](https://trunkrs.dev/) - WASM web application bundler
- **Package Manager**: [Nix](https://nixos.org/) - Reproducible builds and dev environments

## License

This project is open source and available under the [MIT License](LICENSE).

---

*Built with ❤️ and Rust*
