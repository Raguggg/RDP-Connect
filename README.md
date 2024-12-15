# RDP-Connect

RDP-Connect is a lightweight and user-friendly GUI application for Linux, designed to simplify remote desktop connections using the powerful xfreerdp API. I developer for me personal use it may also have bug

## Features
- Easy-to-use graphical interface for managing RDP connections.
- Input fields for server IP, username, and password.
- Seamless integration with xfreerdp for backend connectivity.
- Lightweight and efficient, designed specifically for Linux environments.

## Requirements
- **xfreerdp** must be installed on your system.
- A Linux-based operating system.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Raguggg/RDP-Connect.git
   cd RDP-Connect
   ```
2. Build the application using Cargo:
   ```bash
   cargo build --release
   ```
3. Run the application:
   ```bash
   ./target/release/rdp_connect
   ```

## Usage
1. Launch the app.
2. Enter the **Server IP**, **Username**, and **Password** in the provided fields.
3. Click the **Connect** button to initiate a remote desktop connection.

## Adding an Icon (Optional)
To add a custom icon for the application on Linux:
1. Save your icon as `rdp_connect.png`.
2. Move the icon to the appropriate location:
   ```bash
   sudo mv rdp_connect.png /usr/share/icons/
   ```
3. Create a `.desktop` file:
   ```bash
   sudo nano /usr/share/applications/rdp_connect.desktop
   ```
   Add the following content:
   ```
   [Desktop Entry]
   Name=RDP-Connect
   Comment=Lightweight GUI for xfreerdp
   Exec=/path/to/rdp_connect
   Icon=rdp_connect
   Terminal=false
   Type=Application
   Categories=Network;
   ```
4. Save and close the file. Your application will now appear in the application menu.

## Credits
Special thanks to the developers of **xfreerdp** for providing a robust and reliable RDP backend.


