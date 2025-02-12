# Kewr Panel

A system monitoring dashboard for managing system resources, PM2 processes, Node.js applications, and databases.

## System Requirements

- Operating System: Linux, macOS, or Windows
- Rust toolchain (latest stable version)
- Node.js and npm (for PM2 functionality)
- Sufficient permissions to access system metrics

## Installation

1. Clone the repository
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Install PM2 (optional, for process management):
   ```bash
   npm install -g pm2
   ```

## Directory Structure

Kewr Panel uses the following directory structure:

- `/opt/kewr-panel`: Base directory for the application
- `/var/lib/kewr-panel`: WWW directory for web assets
- `/var/log/kewr-panel`: Log files directory
- `/etc/kewr-panel`: Configuration directory
- `/var/run/kewr-panel`: Runtime directory

## Configuration

1. Ensure the required directories exist and have appropriate permissions:

   ```bash
   sudo mkdir -p /opt/kewr-panel /var/lib/kewr-panel /var/log/kewr-panel /etc/kewr-panel /var/run/kewr-panel
   sudo chown -R <user>:<group> /opt/kewr-panel /var/lib/kewr-panel /var/log/kewr-panel /etc/kewr-panel /var/run/kewr-panel
   ```

2. Copy the built binary to the installation directory:
   ```bash
   sudo cp target/release/panel /opt/kewr-panel/
   ```

## Running the Application

### Manual Start

```bash
/opt/kewr-panel/panel
```

### Systemd Service (Linux)

1. Create a systemd service file `/etc/systemd/system/kewr-panel.service`:

```ini
[Unit]
Description=Kewr Panel System Monitor
After=network.target

[Service]
Type=simple
User=<user>
Group=<group>
ExecStart=/opt/kewr-panel/panel
Restart=on-failure
WorkingDirectory=/opt/kewr-panel

[Install]
WantedBy=multi-user.target
```

2. Enable and start the service:

```bash
sudo systemctl enable kewr-panel
sudo systemctl start kewr-panel
```

## Features

### System Monitoring

- CPU usage tracking
- Memory usage monitoring
- Disk space utilization

### PM2 Integration

- Process list viewing
- Process status monitoring
- Version information

### Node.js Monitoring

- Node.js version information
- Runtime statistics

### Database Monitoring

- Database connection status
- Performance metrics

## Troubleshooting

1. **Permission Issues**

   - Ensure the user running the application has appropriate permissions for all directories
   - Check system logs: `/var/log/kewr-panel/`

2. **Service Won't Start**

   - Verify directory permissions
   - Check systemd logs: `journalctl -u kewr-panel`

3. **PM2 Integration Issues**
   - Ensure PM2 is installed globally
   - Verify PM2 permissions

## Development

### Prerequisites

- Rust (latest stable)
- Cargo
- Node.js and npm (for PM2 features)

### Building from Source

```bash
cargo build
```

### Running in Development Mode

```bash
cargo run
```

## Environment Configuration

Kewr Panel uses environment variables to configure its behavior in different environments (development or production). Create a `.env` file based on the provided `.env.example`:

```bash
cp .env.example .env
```

Key environment variables:

- `ENV`: Set to either `development` or `production`
- `*_DIR`: Directory paths for various application components
- `DEV_*_DIR`: Development environment directory paths
- `LOG_LEVEL`: Logging verbosity
- `PORT`: Server port number
- `HOST`: Server host address

In development mode, the application uses local directories specified by `DEV_*_DIR` variables instead of system directories.

## License

This project is proprietary software of Kewr Digital.

## Support

For support and bug reports, please contact the Kewr Digital support team.
