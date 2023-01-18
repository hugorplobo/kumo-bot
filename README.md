# Kumo bot

![Screenshot demonstration](https://github.com/hugorplobo/kumo-bot/raw/main/assets/screenshot.png)

A telegram bot to save and access in any chat your files. You can check it online [here](https://t.me/kumoo_bot) (@kumoo_bot).

## How it works
Kumo consists of 3 parts: a telegram polling service (the bot itself), an [API](https://github.com/hugorplobo/kumo-api) for managing the files and a [web app](https://github.com/hugorplobo/kumo-web) that integrates with telegram.

Each file send in telegram has a unique id, the bot only store the files id and name of a user and resend the file using this id when requested.

## Usage
| Use case | How to execute |
| ---------- | ------------ |
| Get a help message | Use the **/help** command |
| Save a file | Just send the file in the bot chat |
| List your files | Use the **/list** command |
| Remove a file | The **/list** command will give you a custom command for managing each file like **/id*123***, click in this command and a remove button will appear
| Access in other chats | Type @kumoo_bot in any chat to see your files. You can search specific queries and click any file to send it in that chat |
| Open web app | Use the /web command and click on the button. This is more stable on mobile clients |
