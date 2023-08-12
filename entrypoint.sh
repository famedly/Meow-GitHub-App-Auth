#!/bin/sh

echo "$MEOWGHAUTH_KEY" | base64 -d --ignore-garbage > key
echo GITHUB_APP_TOKEN=$(meow-github-app-auth --app-id $MEOWGHAUTH_APP_ID --installation-id $MEOWGHAUTH_INSTALLATION_ID --key-path key) >> "$GITHUB_OUTPUT"
rm -rf key
