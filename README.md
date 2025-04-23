# Flappy Bevy: A Rust-Powered Flappy Bird Adventure!

[![Bevy engine version](https://img.shields.io/badge/Bevy-latest-blue)](https://bevyengine.org)
[![Rust](https://img.shields.io/badge/Rust-latest-orange)](https://www.rust-lang.org)

**Flappy Bevy** is a classic Flappy Bird game reimagined in Rust, built with the fantastic [Bevy game engine](https://bevyengine.org). This project started as an exploration inspired by the simplicity and educational value of games built with libraries like [Bracket-Lib](https://github.com/amethyst/bracket-lib) but leverages Bevy's modern ECS architecture and powerful features.

This is a work in progress, and I'll be continuously adding new features, polishing the gameplay, and keeping it up-to-date with the latest Bevy releases.

## Features (Current & Planned)

**Current:**

* 🐦 Basic flapping bird mechanics.
* <0xF0><0x9F><0xAA><0xB0> Simple obstacle (pipe) generation.
* 👆 Basic keyboard input for flapping (Spacebar or 'P' key).
* 📜 Simple main menu with options to play and exit.
* 💨 Basic game state management (MainMenu, Playing, GameOver).
* 🎨 Rudimentary visual elements using Bevy's UI system.

**Planned:**

* <0xF0><0x9F><0x9B><0xA2> Scoring system.
* 💥 Collision detection.
* 🔊 Sound effects and background music.
* 🖼️ More polished visual assets (sprites, backgrounds).
* ⚙️ Settings menu.
* 🏆 Leaderboard integration (maybe!).
* 📱 Touchscreen input support.
* 🌐 WebAssembly build for playing in the browser.
* And much more as inspiration strikes and Bevy evolves!

## Getting Started

### Prerequisites

* **Rust:** Ensure you have the latest stable version of Rust installed. You can get it from [rustup.rs](https://rustup.rs/).
* **Cargo:** Rust's package manager, which comes with Rust.

### Running the Game

1.  **Clone the repository:**
    ```bash
    git clone <YOUR_REPOSITORY_URL>
    cd flappy_bevy
    ```
    *(Replace `<YOUR_REPOSITORY_URL>` with the actual URL of your repository)*

2.  **Run the game using Cargo:**
    ```bash
    cargo run --release
    ```
    The `--release` flag is recommended for better performance.

## Controls

* **Spacebar**: Flap the bird.
* **Escape (ESC)**: Exit the main menu (currently might quit the game, this will be refined).

## Architecture

This project utilizes Bevy's Entity-Component-System (ECS) architecture. Key aspects include:

* **Entities:** Represent game objects (bird, pipes, UI elements).
* **Components:** Data associated with entities (position, velocity, graphics, UI layout).
* **Systems:** Logic that operates on entities based on their components (movement, spawning, input handling, UI updates).
* **States:** Manages different phases of the game (MainMenu, Playing, GameOver).

The UI is built using Bevy's built-in UI system, leveraging `NodeBundle` and `TextBundle` for creating the menu and in-game displays.

## Contributing

As this is primarily a personal learning project, I'm not actively seeking external contributions at this time. However, if you have suggestions or find issues, feel free to open an issue on GitHub. Your feedback is always welcome!

## Staying Updated

This project will be updated periodically as I add new features and when new versions of Bevy are released. Follow this repository to stay informed about the latest changes and progress!

## License

This project is currently unlicensed. See the `LICENSE` file for more details when a license is chosen.

## Acknowledgements

* The **Bevy Engine** community for their amazing work and support.
* The creators of **Flappy Bird** for the original inspiration.
* The **Bracket-Lib** developers for inspiring a Rust-based game development journey.

Enjoy playing Flappy Bevy! 🐦🚀