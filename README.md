# NotAudio - Simple Audio API Server

A simple Rust API server that plays audio files from a `sounds` folder.

## Setup

1. Place your audio files (WAV, MP3, etc.) in the `sounds/` folder
2. Run the server: `cargo run`
3. The server will start on `http://localhost:3030`

## API Endpoints

### Play a sound
```
POST /play
Content-Type: application/json

{
  "sound": "filename.wav"
}
```

### List available sounds
```
GET /sounds
```

## Example Usage

```bash
# List available sounds
curl http://localhost:3030/sounds

# Play a sound
curl -X POST http://localhost:3030/play \
  -H "Content-Type: application/json" \
  -d '{"sound": "beep.wav"}'
```

## Requirements

- Audio files should be placed in the `sounds/` folder
- Supports common audio formats (WAV, MP3, FLAC, etc.)
- Designed for Windows but should work on other platforms