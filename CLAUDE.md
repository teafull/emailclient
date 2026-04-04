# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Tauri v2 desktop email client application with a Vue 3 frontend and Rust backend. The app provides mail viewing, composing, contacts management, and AI-assisted draft generation.

## Development Commands

```bash
npm run dev        # Start Vite dev server (http://localhost:1420)
npm run build      # Build Vue frontend to dist/
npm run preview    # Preview built frontend
npm run tauri      # Run Tauri desktop application
```

## Architecture

### Frontend (Vue 3 + Element Plus)
- **Framework**: Vue 3 with Composition API (`<script setup>`)
- **UI Library**: Element Plus for components
- **State**: Reactive refs with localStorage persistence
- **Views**:
  - `MailClient.vue` - Main shell with sidebar, mail list, and todo panel
  - `ComposeMail.vue` - Mail composition dialog
  - `ContactsView.vue` - Contacts management with LDAP sync
  - `SettingsView.vue` - User settings and LLM configuration

### Backend (Rust/Tauri)
- **Location**: `src-tauri/`
- **Entry**: `src-tauri/src/main.rs` calls `emailclient_lib::run()`
- **Commands**: Currently minimal - single `greet` command. Add new Tauri commands in `src-tauri/src/lib.rs` with `#[tauri::command]`.

### Data Persistence
Settings, contacts, labels, and drafts are stored in browser `localStorage` with keys:
- `mailSettings` - User settings including LLM config
- `mailContacts` - Contact list
- `mailContactsLdap` - LDAP sync settings
- `mailLabels` - Mail labels
- `mailDrafts` - Saved drafts

## Key Implementation Details

### Custom Window Controls
The app uses a frameless window (`decorations: false` in tauri.conf.json). Window dragging is enabled via `data-tauri-drag-region` attributes. Custom minimize/maximize/close buttons are implemented in `MailClient.vue` using `@tauri-apps/api/window`.

### AI Draft Generation
LLM integration is configured in settings (OpenAI-compatible). The `generateMailContent` function in `MailClient.vue` calls the configured endpoint to generate email drafts based on subject and tone preferences.

### LDAP Contacts Sync
Contacts view supports syncing with LDAP directories. Settings include host, port, baseDN, bindDN, and auto-sync options stored in `mailContactsLdap`.

## Configuration Files
- `vite.config.js` - Vite with Vue plugin, Tauri server on port 1420
- `src-tauri/tauri.conf.json` - Tauri app config (frameless transparent window)
- `src-tauri/Cargo.toml` - Rust dependencies (tauri, serde, serde_json)
- `package.json` - Node dependencies (vue, element-plus, @tauri-apps/api)
