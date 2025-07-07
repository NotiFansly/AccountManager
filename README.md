<div align="center">
  <img src="https://github.com/NotiFansly/AccountManager/blob/master/src-tauri/icons/icon.png" alt="NotiFansly AccountManager Logo" width="128" height="128">
  <h1>NotiFansly AccountManager</h1>
  <p><strong>A secure, open-source desktop tool to sync your Fansly data with third-party services.</strong></p>
  <p>
    <a href="https://github.com/NotiFansly/AccountManager/releases/latest">
      <img src="https://img.shields.io/github/v/release/NotiFansly/AccountManager?style=for-the-badge&logo=github&color=363a4f&logoColor=D9E0EE&labelColor=494d64" alt="Latest Release"/>
    </a>
    <a href="https://github.com/NotiFansly/AccountManager/actions/workflows/release.yml">
      <img src="https://img.shields.io/github/actions/workflow/status/NotiFansly/AccountManager/release.yml?branch=main&style=for-the-badge&logo=githubactions&logoColor=D9E0EE&label=Build&color=88C0D0" alt="Build Status"/>
    </a>
  </p>
</div>

## About The Project

NotiFansly AccountManager was created for the [NotiFansly](https://creator.notifansly.xyz) platform to give creators a secure way to synchronize their data (like followers, subscribers, and subscription tiers) with our services.

We believe creators should never have to hand over their account credentials. This tool runs directly on your computer, using your Fansly authorization token locally to fetch data and send it to a service of your choice using a secure `sync_key`.

**Your Fansly token is never stored or sent anywhere except directly to Fansly's API from your own machine.**

## How It Works

1.  **Authenticate**: You provide your Fansly auth token to the app, which is stored securely on your computer.
2.  **Link Account**: The app communicates with the third-party service, NotiFansly, to create a linked account.
3.  **Sync Data**: The app uses your token to fetch the latest data from Fansly, then sends it to the service. The service never sees your Fansly token.

This tool is open-source and can be adapted for any developer or service that wishes to integrate with Fansly data in a secure, creator-first manner.

## Getting Started & Installation

You can download the latest version of NotiFansly AccountManager for Windows, macOS, and Linux from our **[Releases Page](https://github.com/NotiFansly/AccountManager/releases/latest)**.

For detailed instructions on how to use FanslySync with the NotiFansly platform, please visit our documentation:

**📖 [View the Full Documentation](https://creator.notifansly.xyz/docs/creator-platform/getting-started#step-1-the-desktop-sync-app)**
## License

This project is distributed under the MIT License. See `LICENSE` for more information.
