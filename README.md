# Mail
This is a Cross platform mail application. This was intended for my use on Linux to replace Geary because of design issues I had with it.

### Features:
- [] Send Mail
- [] Receive Mail
- [X] View Mail
- [X] BCC Mail
- [x] CC Mail
- [] Delete Mail
- [] View Attachments
- [] Download Attachments
- [X] View HTML Mail
- [X] View Plain Text Mail
- [] View Mail Headers
- [] View Mail Source
- [] View Mail Raw
- [] View Mail Flags
- [] View Mail Size
- [X] View Mail Date
- [X] View Mail Subject
- [X] View Mail From
- [] Add ICS Calender
- [] Offset Timezone for ICS Calender

# Start Developing
## Install Dependencies
```bash
cargo install tauri-cli
npm install
```

## Run
```bash
cargo tauri dev
```

### Node Version
More modern versions tend to have issues building with tauri. I recommend using the following version:

Errors that may occur:
- `Illegal instruction (core dumped)`
```
v18.19.0
```