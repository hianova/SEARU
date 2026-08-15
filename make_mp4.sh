#!/bin/bash
echo "🎬 Packaging SEARU Tracks into MP4..."

mkdir -p release/videos

for svg in release/*.svg; do
    filename=$(basename -- "$svg")
    name="${filename%.*}"
    
    echo "🖼️ Rendering Cover Art for $name..."
    # Convert SVG to PNG using macOS built-in QuickLook engine
    qlmanage -t -s 1080 -o release "$svg" > /dev/null 2>&1
    
    png="release/${filename}.png"
    flac="release/${name}.flac"
    wav="release/${name}.wav"
    
    # Prefer FLAC if exists, otherwise fallback to WAV
    audio=$flac
    if [ ! -f "$audio" ]; then
        audio=$wav
    fi
    
    if [ -f "$png" ] && [ -f "$audio" ]; then
        echo "🎥 Muxing $name into MP4..."
        # Mux image and audio into a standard h264/AAC MP4 video
        # We add -ac 2 to ensure AAC encoder accepts the mono track by duplicating it to stereo
        ffmpeg -y -loop 1 -framerate 1 -i "$png" -i "$audio" -c:v libx264 -preset medium -tune stillimage -crf 18 -c:a aac -ac 2 -b:a 320k -pix_fmt yuv420p -shortest "release/videos/${name}.mp4" -loglevel error
        
        # Cleanup temporary PNG
        rm "$png"
        echo "✅ Created release/videos/${name}.mp4"
    else
        echo "⚠️ Missing files for $name (Checked $audio and $png)"
    fi
done

echo "🎉 All tracks have been packaged as videos in the release/videos/ directory!"
