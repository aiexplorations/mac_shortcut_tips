# Mac Shortcut Tips App
# Mac Shortcut Tips

A native macOS application that provides quick access to essential keyboard shortcuts. The app lives in your dock and offers an intuitive interface to browse through various keyboard shortcuts using simple left/right navigation.

## Features

- Native macOS app built with Rust using iced-rs GUI framework
- Dock-based quick access
- Simple left/right navigation through shortcut categories
- Clean, minimal interface following macOS design guidelines
- Categorized shortcuts for different applications and system functions

## Requirements

- macOS 10.7 or later
- Rust 1.70.0 or later
- Cargo (Rust's package manager)

## Installation

### From Release
1. Download the latest release from the Releases page
2. Drag `MacShortcutTips.app` to your Applications folder
3. Launch the app from Applications or Spotlight

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/[username]/mac_shortcut_tips.git
   cd mac_shortcut_tips
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Development

The project uses:
- Rust programming language
- iced-rs for the user interface
- Cargo for dependency management
- MVC (Model-View-Controller) architecture

### Project Structure
```
src/
├── main.rs
├── app.rs
├── models/
├── views/
├── controllers/
└── utils/
resources/
└── shortcuts/
tests/
```

## Usage

1. Launch the app - it will appear in your dock
2. Click the app icon to show the shortcuts window
3. Use left/right arrow buttons or keyboard arrows to navigate
4. Click the category name to filter shortcuts
5. Use ⌘Q to quit the application

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details
