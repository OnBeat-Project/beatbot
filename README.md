# 🎵 OnBeat Bot

A modern Discord music bot built with Rust, featuring high-quality audio playback powered by Lavalink and Spotify integration.

> [!NOTE]
> This is a private bot project. The code is open-source for transparency and bug reporting purposes only. **Please do not host your own instance of this bot.** Contributions to fix bugs and improve code quality are welcome.

## ✨ Features

- 🎵 **High-Quality Audio Playback** - Crystal clear audio powered by Lavalink
- 🎧 **Spotify Integration** - Search and play tracks directly from Spotify
- 🔍 **Smart Autocomplete** - Intelligent search suggestions as you type
- 🎼 **Queue Management** - Add songs and playlists to your queue
- ⚡ **Fast & Efficient** - Built with Rust for optimal performance
- 🎨 **Beautiful Embeds** - Rich, colorful Discord embeds for all responses

## 📋 Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `/join` | Makes the bot join your voice channel | `/join` |
| `/play` | Plays a song or playlist from Spotify/URL | `/play <query or URL>` |
| `/skip` | Skips the currently playing track | `/skip` |

## 🔗 Add to Your Server

Want to use OnBeat Bot? Add the official instance to your server:

**[Invite OnBeat Bot](https://discord.com/oauth2/authorize?client_id=916373041460703282)**

---

## 👨‍💻 For Developers

This section is for developers who want to contribute bug fixes and improvements.

### Prerequisites

- **Rust** (latest stable version)
- **Lavalink Server** (for audio processing)
- **Java 17+** (for Lavalink)

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/yourusername/onbeat-bot.git
   cd onbeat-bot
   ```

2. **Set up Lavalink for testing**
   
   Download Lavalink from [GitHub Releases](https://github.com/lavalink-devs/Lavalink/releases)
   
   Create an `application.yml` file:
   ```yaml
   server:
     port: 2333
     address: 0.0.0.0
   lavalink:
     server:
       password: "youshallnotpass"
       sources:
         youtube: true
         bandcamp: true
         soundcloud: true
         twitch: true
         vimeo: true
         http: true
         local: false
       bufferDurationMs: 400
       frameBufferDurationMs: 5000
       youtubePlaylistLoadLimit: 6
       playerUpdateInterval: 5
       youtubeSearchEnabled: true
       soundcloudSearchEnabled: true
       gc-warnings: true
   
   metrics:
     prometheus:
       enabled: false
       endpoint: /metrics
   
   sentry:
     dsn: ""
     environment: ""
   
   logging:
     file:
       path: ./logs/
     level:
       root: INFO
       lavalink: INFO
   
   plugins:
     - dependency: "dev.lavalink.youtube:youtube-plugin:1.7.2"
       snapshot: false
   ```

3. **Configure environment variables**
   
   Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` with your test bot credentials:
   ```env
   LAVA_HOST=localhost:2333
   LAVA_PASSWORD=youshallnotpass
   BOT_TOKEN=your_test_bot_token_here
   ```

4. **Build and test**
   ```bash
   cargo build
   cargo run
   ```

### Testing Your Changes

Start your Lavalink server:
```bash
java -jar Lavalink.jar
```

Then run the bot and test your changes thoroughly before submitting a pull request.

## 🏗️ Project Structure

```
onbeat-bot/
├── src/
│   ├── commands/           # Bot commands
│   │   ├── join.rs        # Join voice channel
│   │   ├── play.rs        # Play music
│   │   ├── skip.rs        # Skip track
│   │   └── mod.rs
│   ├── utils/             # Utility functions
│   │   ├── voicechannel.rs
│   │   └── mod.rs
│   ├── music_events.rs    # Lavalink event handlers
│   └── main.rs            # Entry point
├── Cargo.toml             # Dependencies
├── .env.example           # Environment variables template
├── .gitignore
└── README.md
```

## 🛠️ Technologies Used

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Poise](https://github.com/serenity-rs/poise)** - Discord bot framework
- **[Serenity](https://github.com/serenity-rs/serenity)** - Discord API library
- **[Songbird](https://github.com/serenity-rs/songbird)** - Voice client
- **[Lavalink-rs](https://github.com/vicky5124/lavalink-rs)** - Lavalink client
- **[Lavalink](https://github.com/lavalink-devs/Lavalink)** - Audio player server
- **[Tokio](https://tokio.rs/)** - Async runtime

## 🎯 Usage Examples

### Playing a Song
```
/play never gonna give you up
```

### Playing from URL
```
/play https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT
```

### Playing a Playlist
```
/play https://open.spotify.com/playlist/37i9dQZF1DXcBWIGoYBM5M
```

## 🐛 Troubleshooting

### Bot doesn't join voice channel
- Ensure you're in a voice channel before using `/join` or `/play`
- Check bot permissions (Connect, Speak)
- Verify Lavalink is running

### No audio playback
- Check Lavalink logs for errors
- Ensure Lavalink plugins are properly loaded
- Verify your internet connection

### Commands not appearing
- Wait a few minutes for Discord to register slash commands
- Try kicking and re-inviting the bot
- Check bot has `applications.commands` scope

## 📝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- [Lavalink](https://github.com/lavalink-devs/Lavalink) for the amazing audio server
- [Serenity](https://github.com/serenity-rs/serenity) community for Discord API support
- All contributors who help improve this project

## 📞 Support

If you need help or have questions:

- Open an [issue](https://github.com/OnBeat-Project/onbeat-bot/issues)
- Join our [Discord server](https://discord.gg/8HCVTjj8Q)
- Check existing issues and discussions

## 🗺️ Roadmap

- [x] Queue viewing command
- [ ] Loop/repeat functionality
- [ ] Volume control
- [ ] Song seeking
- [ ] Lyrics display
- [ ] DJ role permissions
- [ ] Web dashboard
- [ ] More music sources (SoundCloud, Apple Music)

---

Made with ❤️ and Rust
