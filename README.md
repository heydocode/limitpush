Here's the complete `README.md` for your LimitPush project:

```markdown
# LimitPush

LimitPush is a cross-platform 3D game developed using Bevy. This project focuses on enhancing gameplay performance and implementing an engaging player camera system.

## IMPORTANT
PLEASE TAKE IF NEEDED THE CAMERA SYSTEM WHICH IS LITERALLY AWESOME! Don't forget to pick the `MainCamera` struct with its fields and make this component attached to the player `PbrBundle` (or another Bundle if you've done such). Please enjoy the template and feel free to create an issue if you need help or if there's a problem — I'll be happy to help! :) Also, please commit if you have some improvements to propose! I really love Bevy, and I hope it becomes even more ergonomic!

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Camera System](#camera-system)
- [Contributing](#contributing)
- [License](#license)

## Features

- Cross-platform compatibility
- Smooth camera follow mechanics
- Customizable camera settings

## Getting Started

To get started with LimitPush, ensure you have the following prerequisites:

1. [Rust](https://www.rust-lang.org/tools/install) (with Cargo)
2. [Bevy](https://bevyengine.org)

Clone the repository:

```bash
git clone https://github.com/yourusername/limitpush.git
cd limitpush
```

## Usage

After cloning the repository, you can run the project using:

```bash
cargo run
```

Feel free to explore the code, make changes, and enhance the gameplay!

## Camera System

The camera system in LimitPush is designed to follow the player smoothly while **not** avoiding obstacles (it's an issue, please commit if you have time to). The `MainCamera` struct includes fields to customize its behavior. Here's a brief overview of the struct:

```rust
#[derive(Component)]
pub struct MainCamera {
    pub min_distance: f64,
    pub min_speed: f64,
    pub max_distance: f64,
    pub max_speed: f64,
    pub ideal_distance: f64,
}
```

Integrate this camera system into your player entity for an immersive experience!

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to create a pull request or open an issue. Let's make LimitPush even better together!

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.