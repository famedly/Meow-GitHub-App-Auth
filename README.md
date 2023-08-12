# GitHub App Auth

A simple actions helper that allows you to generate an app auth token. 

The action takes 3 inputs:

- `app-id`: GitHub App ID
- `installation-id`: GitHub App Installation ID
- `key-base64`: GitHub App Key encoded as Base64

The token generated is valid for 2 minutes.

The output is stored in `outputs.GITHUB_APP_TOKEN`
