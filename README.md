# Pomodoro App

A Pomodoro timer application built with a modern stack focusing on performance and simplicity.

## Features

*   **Customizable Timers**: Set custom durations for Work sessions, Short Breaks, and Long Breaks.
*   **Neobrutalism UI**: A clean, soft-colored user interface with slightly curved corners for a modern look.
*   **System Tray Integration**: Easily control and monitor the timer from your system tray.
*   **Native Notifications**: Get system-level notifications when sessions begin or end.

## Architecture

This application uses a unique architecture emphasizing separation of concerns and performance:

*   **Backend (Rust)**: The core of the application is written in Rust using Tauri v2. It acts as the single source of truth for background processes, timer state management, system tray integration, and notifications.
*   **Frontend (Vanilla Web Technologies)**: The user interface is built entirely with Vanilla HTML, CSS, and JavaScript. No JavaScript frameworks or bundlers are used, keeping the footprint minimal and fast.
*   **Communication**: The Rust backend emits events to the vanilla JS frontend for UI updates, ensuring the UI always reflects the true state managed by the backend.

## Tech Stack

*   [Tauri v2](https://v2.tauri.app/): Framework for building tiny, fast binaries.
*   [Rust](https://www.rust-lang.org/): Systems programming language for the backend.
*   Vanilla HTML/CSS/JS: Native web technologies for the frontend.

## Getting Started

### Prerequisites

Ensure you have the following installed:

*   [Node.js](https://nodejs.org/) (v20+ recommended)
*   [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
*   Tauri dependencies for your OS (see [Tauri documentation](https://v2.tauri.app/start/prerequisites/))

### Installation

1.  Clone the repository.
2.  Install frontend dependencies (Tauri CLI):
    ```bash
    npm install
    ```

### Development

To start the development server with hot-reloading:

```bash
npm run tauri dev
```

### Build

To build the application for release:

```bash
npm run tauri build
```

## CI/CD Pipeline

This project utilizes GitHub Actions for continuous integration and continuous deployment (CI/CD).

*   **Workflow**: The pipeline is configured to automatically build and release the application for Windows.
*   **Triggers**: A new release is triggered whenever a tag starting with `v` (e.g., `v1.0.0`) is pushed to the repository. It can also be triggered manually.
*   **Environment**: The build process runs on a `windows-latest` runner, utilizing Node 20 and the stable Rust `x86_64-pc-windows-msvc` toolchain.
