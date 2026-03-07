# Mail

Mail is now a Dioxus desktop application. The Vue and Tauri stack has been removed, and the canonical app lives in the `application/` crate.

## Current Scope

- View mail across discovered accounts
- Browse mailbox folders
- Read HTML and plain text messages
- Inspect sender, recipient, subject, and date metadata

## Development

Install the Dioxus CLI once:

```bash
cargo install dioxus-cli
```

Run the desktop app from the Dioxus crate:

```bash
cd application
dx serve --platform desktop
```

For a compile-only validation pass:

```bash
cd application
cargo check
```

## Notes

- Tailwind is configured inside `application/`.
- Linux account discovery currently uses GNOME Online Accounts.
- Root-level Node and Tauri tooling is no longer part of the project.