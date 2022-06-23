# MOTD
A project to show a message on terminal start.

## Options
The config file is located in your config directory after running the daemon once.
Most likely: ~/.config
```json
{
    "key": "Get at https://www.weatherapi.com/",
    "city": "Brussels",
    "message_type": "weather" //options are weather and reddit
}
```
## Implement
How you implement it is up to you.
Personally I have a crontab running every 5 minutes that runs the daemon.
Then I just run the client in  ~/.bashrc .