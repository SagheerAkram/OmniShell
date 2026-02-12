# Command Reference

## Messaging
- `msg @user <text>`: Send message.
- `read @user`: Read history.
- `reply <id> <text>`: Reply to specific message.

## Groups
- `group create <name> @users...`: Create group.
- `group msg <name> <text>`: Message group.

## Files
- `send @user <file>`: Send file.
- `resume <transfer_id>`: Resume transfer.

## Security
- `panic`: Wipe keys/memory.
- `mirage --mode <mode>`: Fake UI.
- `sentry --arm`: Physical surveillance.

## Network
- `agility --monitor`: Frequency hopping.
- `mole --target <ip>`: ICMP tunnel.
- `sonar --send <msg>`: Ultrasonic data.
