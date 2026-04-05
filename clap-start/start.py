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
        "-n",
        "-a", 
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
                fft_data = np.abs(np.fft.rfft(audio_data))
                freqs = np.fft.rfftfreq(CHUNK, 1.0/RATE)

                high_freq_energy = np.sum(fft_data[freqs > 1500])
                total_energy = np.sum(fft_data)
                
                if total_energy == 0: continue 
                
                high_freq_ratio = high_freq_energy / total_energy

                # Check if the sound is predominantly high-frequency
                FREQ_RATIO = 0.6 
                
                if high_freq_ratio > FREQ_RATIO:
                    current_time = time.time()
                    time_since_last = current_time - last_clap_time

                    if clap_count == 1:
                        if MIN_DELAY < time_since_last < MAX_DELAY:
                            for num in audio_data:
                                print(num); 
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

