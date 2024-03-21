## About the project

This is a telegram job board bot with a web interface. As a last ditch effort (🥲) to use rust on the backend, I decided to build a telegram job board bot that leverages telegram for crud operations.
## Getting Started

To get started here's the username of the telegram bot:

```
@gram_job_board_bot
```

The backend might be asleep if you hop onto to telegram and find that it isn't responding to your texts 😙.

If you want to test the app, clone the app, create a Secrets.toml file, head over to telegram.
Search for BotFather. Create your own telegram bot. A teloxide token will be provided for that bot. Type the following into your Secrets.toml file.

```
TELOXIDE_TOKEN="1234:blahblahblah"
```

The TELOXIDE_TOKEN will look like the above. Don't copy the one above. Its fake.

Then login to shuttle with their CLI, and create a project on shuttle using the following cmd:

```
cargo shuttle project start
```

Then deploy it ;

```
cargo shuttle deploy
```

## Special Thanks

Special thanks to teloxide, askama and all the other crates used in this project. I relied on the backs of giants 👏.
