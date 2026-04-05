# What is this? 

An embeded software project,so that when I clap my hands twice it opens my terminal (ghostty). 

## Installation & Setup

### 1. Install System Dependencies
The Python audio library (PyAudio) requires the C-library portaudio to interface with your Mac's microphone. Install it via Homebrew:

```bash
brew install portaudio
```

### 2. Set Up the Python Environment

Clone this repository, navigate into the project folder, and create a virtual environment to keep dependencies isolated:

```bash
# Create and activate the virtual environment
python3 -m venv venv
source venv/bin/activate

# Install the required Python packages
# Note: Apple Silicon (M1/M2/M3) Macs may require specific flags to link PyAudio to PortAudio
env LDFLAGS="-L$(brew --prefix portaudio)/lib" CFLAGS="-I$(brew --prefix portaudio)/include" pip install -r requirements.txt
```

### 3. Configure the Background Daemon
macOS uses a .plist file to run scripts in the background. Because launchd runs from the system root, it requires absolute paths to your files.

Open the provided .plist file (e.g., com.yourname.clapper.plist) in a text editor.

Locate the <array> block under the ProgramArguments key.

Update the two path strings to match the exact, absolute paths on your specific machine:

Path 1: The absolute path to the Python executable inside your newly created venv e.g.,

```bash
/Users/yourusername/path/to/project/venv/bin/python
```

Path 2: The absolute path to the start.py script e.g., 

```bash
/Users/yourusername/path/to/project/start.py
```

### 4. Deploy the Daemon
Move your configured .plist file into the macOS LaunchAgents directory, where user-level background services live:

```bash
# Copy the file to the LaunchAgents folder
cp com.yourname.clapper.plist ~/Library/LaunchAgents/

# Tell macOS to load the file and start the daemon
launchctl load ~/Library/LaunchAgents/com.yourname.clapper.plist
```
