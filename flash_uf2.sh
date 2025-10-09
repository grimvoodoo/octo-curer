#!/bin/bash

# Helper script to flash UF2 files to Raspberry Pi Pico
# Handles common read-only mounting issues

UF2_FILE="$1"

if [ -z "$UF2_FILE" ]; then
    echo "Usage: $0 <uf2_file>"
    echo "Example: $0 blinky.uf2"
    exit 1
fi

if [ ! -f "$UF2_FILE" ]; then
    echo "Error: UF2 file '$UF2_FILE' not found"
    exit 1
fi

# Wait for Pico to be detected
echo "Waiting for Pico in BOOTSEL mode..."
while [ ! -e /dev/disk/by-label/RPI-RP2 ]; do
    sleep 1
done

echo "Pico detected!"

# Find the actual device
DEVICE=$(readlink -f /dev/disk/by-label/RPI-RP2)
MOUNT_POINT="/run/media/$USER/RPI-RP2"

echo "Device: $DEVICE"
echo "Mount point: $MOUNT_POINT"

# Wait a moment for auto-mounting
sleep 2

# Check if it's mounted and writable
if mountpoint -q "$MOUNT_POINT"; then
    echo "Testing write access..."
    if touch "$MOUNT_POINT/write_test" 2>/dev/null; then
        rm "$MOUNT_POINT/write_test"
        echo "Write access OK, copying UF2..."
        cp "$UF2_FILE" "$MOUNT_POINT/"
        echo "✅ Flash complete!"
        exit 0
    else
        echo "⚠️  Mount is read-only, attempting to fix..."
        sudo umount "$MOUNT_POINT"
        sleep 1
        sudo mount -o rw,uid=$(id -u),gid=$(id -g) "$DEVICE" "$MOUNT_POINT"
        
        if cp "$UF2_FILE" "$MOUNT_POINT/" 2>/dev/null; then
            echo "✅ Flash complete after remount!"
            exit 0
        else
            echo "❌ Remount failed, trying direct write..."
        fi
    fi
else
    echo "Not auto-mounted, mounting manually..."
    sudo mkdir -p "$MOUNT_POINT"
    sudo mount -o rw,uid=$(id -u),gid=$(id -g) "$DEVICE" "$MOUNT_POINT"
    
    if cp "$UF2_FILE" "$MOUNT_POINT/" 2>/dev/null; then
        echo "✅ Flash complete!"
        exit 0
    fi
fi

# Last resort: direct write
echo "🔥 Using direct write method..."
sudo dd if="$UF2_FILE" of="$DEVICE" bs=512 conv=sync
echo "✅ Direct flash complete!"