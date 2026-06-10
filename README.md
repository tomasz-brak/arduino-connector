# ArduinoConnector

Quick and easy way to connect an arduino board to your computer. Automaticaly manage projects and discover connected boards!

# Usage


# Common problems

## WSL
If you are trying to connect to a *Serial* device under WSL you need to pass it through
*Example for arduino NANO*
```ps1
usbipd attach --wsl --hardware-id 2341:0043 --auto-attach
```
Remeber to change the hardware-id to your boards hardware-id.

## User group
Add your user to the uucp group to interact with serial devices without `sudo`
```bash
sudo usermod -aG uucp $USER
```

# Note on cross-compatibility
While this tool was developed primary for use on linux the goal is full compatibility with other os's.

