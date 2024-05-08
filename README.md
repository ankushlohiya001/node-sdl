# node-sdl

node-sdl is a Node.js addon providing bindings for SDL (Simple DirectMedia Layer) using the `node-addon-api`. This allows developers to utilize SDL functionality within Node.js applications, enabling the development of multimedia applications, games, and interactive software.

## Features

- **Cross-Platform**: SDL provides a consistent API across multiple platforms, including Windows, macOS, Linux, and more.
- **Low-Level Access**: Gain direct access to audio, keyboard, mouse, joystick, and graphics hardware.
- **Performance**: Utilize hardware-accelerated graphics and audio for optimal performance.
- **Node.js Integration**: Seamlessly integrate SDL functionality into Node.js applications using the `node-addon-api`.

## Installation

Before installing node-sdl, ensure you have the necessary prerequisites:

- Node.js (version 18 or higher)
- npm (Node.js package manager)
- SDL development libraries

To install node-sdl, use npm:

```bash
npm install node-sdl
```

## Usage

Here's a simple example demonstrating how to use node-sdl to create a window:

```javascript
const app = require("node-sdl");

// Create a window
const window = app.createWindow({
  title: "SDL Window",
  width: 800,
  height: 600,
});
// that's it
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

Before contributing, please review our contributing guidelines for instructions on how to contribute to the project.
