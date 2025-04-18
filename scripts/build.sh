#!/bin/bash

# Get and increment version
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | cut -d '"' -f 2)
MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)

# Increment patch version
NEW_PATCH=$((PATCH + 1))
NEW_VERSION="$MAJOR.$MINOR.$NEW_PATCH"

# Update version in Cargo.toml
sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Use new version
VERSION=$NEW_VERSION

# Create build directories
mkdir -p target/dmg
mkdir -p target/release/MacShortcutTips.app/Contents/{MacOS,Resources}

# Build the release version
cargo build --release

# Copy the binary to the app bundle
cp target/release/mac_shortcut_tips "target/release/MacShortcutTips.app/Contents/MacOS/MacShortcutTips"

# Create Info.plist
cat > "target/release/MacShortcutTips.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>MacShortcutTips</string>
    <key>CFBundleDisplayName</key>
    <string>Mac Shortcut Tips</string>
    <key>CFBundleIdentifier</key>
    <string>com.macshortcuttips.app</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleExecutable</key>
    <string>MacShortcutTips</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.7</string>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
</dict>
</plist>
EOF

# Create DMG
DMG_NAME="MacShortcutTips-v$VERSION.dmg"
rm -f "target/dmg/$DMG_NAME"

# Create temporary DMG
hdiutil create -size 32m -fs HFS+ -volname "Mac Shortcut Tips" -o "target/dmg/temp_dmg"
hdiutil attach "target/dmg/temp_dmg.dmg"

# Copy the app bundle
cp -R "target/release/MacShortcutTips.app" "/Volumes/Mac Shortcut Tips/"

# Create Applications symlink
ln -s /Applications "/Volumes/Mac Shortcut Tips/Applications"

# Detach the temporary DMG
hdiutil detach "/Volumes/Mac Shortcut Tips"

# Convert temporary DMG to compressed DMG
hdiutil convert "target/dmg/temp_dmg.dmg" -format UDZO -o "target/dmg/$DMG_NAME"
rm "target/dmg/temp_dmg.dmg"

# Generate changelog if git is available
if command -v git &> /dev/null; then
    if [ ! -f CHANGELOG.md ]; then
        echo "# Changelog" > CHANGELOG.md
    fi

    # Get the latest git tag
    LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "No previous tags")

    if [ "$LATEST_TAG" != "No previous tags" ]; then
        # Get commits since last tag
        echo -e "\n## Version $VERSION ($(date '+%Y-%m-%d'))" >> CHANGELOG.md
        git log $LATEST_TAG..HEAD --pretty=format:"* %s" >> CHANGELOG.md
    else
        # Get all commits if no tags exist
        echo -e "\n## Version $VERSION ($(date '+%Y-%m-%d'))" >> CHANGELOG.md
        git log --pretty=format:"* %s" >> CHANGELOG.md
    fi

    # Create a new git tag
    git tag -a "v$VERSION" -m "Release version $VERSION"
fi

echo "Build complete! DMG file created at target/dmg/$DMG_NAME"
