import pyaudio
import numpy as np
import time
import subprocess

# Configs 

CHUNK = 1024
FORMAT = pyaudio.paInt16
CHANNELS = 1
RATE = 44100
THRESHOLD = 3000
MAX_DELAY = 0.8 
MIN_DELAY = 0.2 

def open_terminal():
    # Using AppleScript to open Ghostty
    subprocess.run([
        "osascript",
        "-e",
        'tell application "Ghostty" to activate'
        ])
    
def main():
    p = pyaudio.PyAudio()
    stream = p.open(
        format=FORMAT,
        channels=CHANNELS,
        rate=RATE,
        input=True,
        frames_per_buffer=CHUNK
        )

    print("Started listening for claps")
    clap_count = 0
    last_clap_time = 0
    
    try: 
        while True:
            data = stream.read(
                    CHUNK,
                    exception_on_overflow=False
                    )
            audio_data = np.frombuffer(
                    data,
                    dtype=np.int16
                    )

            rms = np.sqrt(np.mean(audio_data.astype(np.float32)**2))
            if rms > THRESHOLD:
                current_time = time.time()
                time_since_last = current_time - last_clap_time
                if clap_count == 1:
                    if MIN_DELAY < time_since_last < MAX_DELAY:
                        print("Opening terminal")
                        open_terminal()
                        clap_count = 0
                        time.sleep(1)
                    elif time_since_last >= MAX_DELAY:
                        clap_count = 1 
                        last_clap_time = current_time
                else:
                    clap_count = 1 
                    last_clap_time =current_time
                time.sleep(0.1)

    except KeyboardInterrupt:
        print("Closing app")
    finally: 
        stream.stop_stream()
        stream.close()
        p.terminate()

if __name__ == "__main__":
    main()

