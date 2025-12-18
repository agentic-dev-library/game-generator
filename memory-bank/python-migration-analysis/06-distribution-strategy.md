# Distribution Strategy

## Overview

Python's distribution story has improved dramatically. We can create single-file executables that rival Rust binaries in ease of distribution.

## 1. PyInstaller Approach (Recommended)

### Configuration
```python
# pyinstaller.spec
a = Analysis(
    ['app.py'],
    pathex=[],
    binaries=[],
    datas=[
        ('ai/metaprompts', 'ai/metaprompts'),
        ('assets', 'assets'),
        ('data/games.json', 'data'),
    ],
    hiddenimports=[
        'tiktoken_ext.openai_public',
        'tiktoken_ext',
        'streamlit.web.cli',
        'streamlit.runtime.scriptrunner.magic_funcs',
    ],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[
        'matplotlib',
        'scipy',
        'pandas',  # Unless needed
    ],
    noarchive=False,
)

pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='VintageGameGenerator',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=False,  # No console window
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
    icon='assets/icon.ico'
)
```

### Build Script
```python
# build.py
import os
import shutil
import subprocess
import platform

def build_executable():
    """Build platform-specific executable"""
    
    system = platform.system()
    
    # Clean previous builds
    if os.path.exists('dist'):
        shutil.rmtree('dist')
    if os.path.exists('build'):
        shutil.rmtree('build')
    
    # Platform-specific options
    if system == "Windows":
        icon = "assets/icon.ico"
        name = "VintageGameGenerator.exe"
    elif system == "Darwin":  # macOS
        icon = "assets/icon.icns"
        name = "VintageGameGenerator.app"
    else:  # Linux
        icon = "assets/icon.png"
        name = "VintageGameGenerator"
    
    # Build command
    cmd = [
        "pyinstaller",
        "--onefile",
        "--windowed",
        f"--icon={icon}",
        f"--name={name}",
        "--add-data", f"ai/metaprompts{os.pathsep}ai/metaprompts",
        "--add-data", f"assets{os.pathsep}assets",
        "--hidden-import", "streamlit.web.cli",
        "--hidden-import", "tiktoken_ext.openai_public",
        "--collect-all", "streamlit",
        "--collect-all", "altair",
        "--collect-all", "vega_datasets",
        "app.py"
    ]
    
    subprocess.run(cmd, check=True)
    
    # Post-processing
    if system == "Darwin":
        create_mac_app_bundle()
    elif system == "Windows":
        create_windows_installer()
    
    print(f"Build complete! Executable at: dist/{name}")

if __name__ == "__main__":
    build_executable()
```

## 2. Nuitka Approach (Alternative)

### Benefits
- Compiles to C++
- 2-4x performance improvement
- Smaller executables
- Better obfuscation

### Configuration
```python
# build_nuitka.py
import subprocess
import platform

def build_with_nuitka():
    """Build using Nuitka compiler"""
    
    cmd = [
        "python", "-m", "nuitka",
        "--standalone",
        "--onefile",
        "--enable-plugin=anti-bloat",
        "--enable-plugin=implicit-imports",
        "--include-package-data=streamlit",
        "--include-data-dir=ai/metaprompts=ai/metaprompts",
        "--include-data-dir=assets=assets",
        "--windows-disable-console",
        "--macos-create-app-bundle",
        "--linux-onefile-icon=assets/icon.png",
        "app.py"
    ]
    
    if platform.system() == "Windows":
        cmd.extend([
            "--windows-icon-from-ico=assets/icon.ico",
            "--windows-company-name=VintageGames",
            "--windows-product-name=Vintage Game Generator",
            "--windows-product-version=1.0.0.0"
        ])
    
    subprocess.run(cmd, check=True)
```

## 3. Cross-Platform Packaging

### Windows Installer (NSIS)
```nsis
; installer.nsi
!include "MUI2.nsh"

Name "Vintage Game Generator"
OutFile "VintageGameGenerator-Setup.exe"
InstallDir "$PROGRAMFILES\VintageGameGenerator"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File "dist\VintageGameGenerator.exe"
    File /r "assets"
    
    CreateDirectory "$SMPROGRAMS\Vintage Game Generator"
    CreateShortcut "$SMPROGRAMS\Vintage Game Generator\Vintage Game Generator.lnk" \
                   "$INSTDIR\VintageGameGenerator.exe"
    CreateShortcut "$DESKTOP\Vintage Game Generator.lnk" \
                   "$INSTDIR\VintageGameGenerator.exe"
    
    WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd
```

### macOS App Bundle
```python
# create_mac_bundle.py
def create_mac_app_bundle():
    """Create proper macOS app bundle"""
    
    info_plist = """<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" 
         "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Vintage Game Generator</string>
    <key>CFBundleDisplayName</key>
    <string>Vintage Game Generator</string>
    <key>CFBundleIdentifier</key>
    <string>com.vintagegames.generator</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>"""
    
    # Write Info.plist
    with open("dist/VintageGameGenerator.app/Contents/Info.plist", "w") as f:
        f.write(info_plist)
    
    # Copy icon
    shutil.copy(
        "assets/icon.icns",
        "dist/VintageGameGenerator.app/Contents/Resources/"
    )
    
    # Sign the app (if certificates available)
    subprocess.run([
        "codesign", "--deep", "--force", "--sign", "-",
        "dist/VintageGameGenerator.app"
    ])
```

### Linux AppImage
```bash
#!/bin/bash
# create_appimage.sh

# Download AppImage tools
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons

# Copy executable
cp dist/VintageGameGenerator AppDir/usr/bin/

# Create desktop file
cat > AppDir/usr/share/applications/vintage-game-generator.desktop << EOF
[Desktop Entry]
Name=Vintage Game Generator
Exec=VintageGameGenerator
Icon=vintage-game-generator
Type=Application
Categories=Development;Game;
EOF

# Copy icon
cp assets/icon.png AppDir/usr/share/icons/vintage-game-generator.png

# Create AppRun
cat > AppDir/AppRun << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/VintageGameGenerator" "$@"
EOF
chmod +x AppDir/AppRun

# Build AppImage
./appimagetool-x86_64.AppImage AppDir VintageGameGenerator.AppImage
```

## 4. Web Deployment (Bonus)

### Streamlit Cloud
```yaml
# .streamlit/config.toml
[server]
port = 8501
enableCORS = false
enableXsrfProtection = false

[theme]
primaryColor = "#FF6B6B"
backgroundColor = "#1E1E1E"
secondaryBackgroundColor = "#262626"
textColor = "#FFFFFF"
```

### Deployment Script
```python
# deploy_web.py
import subprocess
import os

def deploy_to_streamlit_cloud():
    """Deploy to Streamlit Cloud"""
    
    # Create requirements.txt from poetry
    subprocess.run([
        "poetry", "export", 
        "--without-hashes", 
        "--format=requirements.txt",
        "--output=requirements.txt"
    ])
    
    # Create secrets.toml template
    secrets_template = """# .streamlit/secrets.toml
# Add your API keys here
OPENAI_API_KEY = "your-key-here"
ANTHROPIC_API_KEY = "your-key-here"
"""
    
    os.makedirs(".streamlit", exist_ok=True)
    with open(".streamlit/secrets.toml.template", "w") as f:
        f.write(secrets_template)
    
    print("Ready for deployment!")
    print("1. Push to GitHub")
    print("2. Connect to Streamlit Cloud")
    print("3. Add secrets in Streamlit Cloud dashboard")
```

## 5. Auto-Update System

### Implementation
```python
# utils/updater.py
import requests
import subprocess
import sys
from packaging import version

class AutoUpdater:
    def __init__(self):
        self.current_version = "1.0.0"
        self.update_url = "https://api.github.com/repos/vintage/generator/releases/latest"
    
    async def check_for_updates(self):
        """Check if updates are available"""
        try:
            response = requests.get(self.update_url)
            latest = response.json()
            latest_version = latest['tag_name'].lstrip('v')
            
            if version.parse(latest_version) > version.parse(self.current_version):
                return {
                    'available': True,
                    'version': latest_version,
                    'download_url': latest['assets'][0]['browser_download_url'],
                    'release_notes': latest['body']
                }
        except:
            pass
        
        return {'available': False}
    
    async def download_and_install(self, download_url):
        """Download and install update"""
        # Implementation depends on platform
        pass
```

## 6. Distribution Channels

### 1. Direct Download
- GitHub Releases
- Project website
- Size: ~50-100MB per platform

### 2. Package Managers
```bash
# Homebrew (macOS)
brew tap vintage/games
brew install vintage-game-generator

# Scoop (Windows)
scoop bucket add vintage https://github.com/vintage/scoop-bucket
scoop install vintage-game-generator

# Snap (Linux)
snap install vintage-game-generator
```

### 3. Steam/Itch.io
- Package as a tool/utility
- Auto-update through platform
- Wider audience reach

## Key Advantages

1. **Single File**: Users download one file, no installation needed
2. **No Dependencies**: Everything bundled, including Python
3. **Fast Startup**: Nuitka-compiled versions start instantly
4. **Small Size**: 50-100MB vs 200MB+ for Rust with all assets
5. **Auto-Update**: Easy to implement self-updating
6. **Cross-Platform**: Same build process for all platforms
7. **Web Option**: Can also run in browser via Streamlit Cloud

## Comparison with Rust

| Aspect | Python | Rust |
|--------|--------|------|
| Build Time | 2-5 minutes | 10-30 minutes |
| Binary Size | 50-100MB | 150-200MB |
| Distribution | Single file | Single file |
| Auto-update | Easy | Complex |
| Web Deploy | Native (Streamlit) | Requires WASM |
| Package Managers | pip, conda, brew | cargo, brew |

The Python distribution story is actually simpler than Rust for this use case!