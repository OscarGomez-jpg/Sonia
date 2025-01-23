from vosk import Model, KaldiRecognizer
import os
import wave
import json
from flask import Flask, request, jsonify

app = Flask(__name__)

# Load Vosk model
model_path = "models/vosk-model-small-en-us-0.15"
if not os.path.exists(model_path):
    print(
        "Please download the model from https://alphacephei.com/vosk/models and unpack as 'model' in the current folder."
    )
    exit(1)

model = Model(model_path)


@app.route("/recognize", methods=["POST"])
def recognize():
    if "audio" not in request.files:
        return jsonify({"error": "No audio file provided"}), 400

    audio_file = request.files["audio"]
    wf = wave.open(audio_file, "rb")
    if (
        wf.getnchannels() != 1
        or wf.getsampwidth() != 2
        or wf.getframerate() not in [8000, 16000, 32000, 44100, 48000]
    ):
        return jsonify({"error": "Audio file must be WAV format mono PCM."}), 400

    rec = KaldiRecognizer(model, wf.getframerate())
    rec.SetWords(True)

    while True:
        data = wf.readframes(4000)
        if len(data) == 0:
            break
        if rec.AcceptWaveform(data):
            result = rec.Result()
        else:
            result = rec.PartialResult()

    result = rec.FinalResult()
    result_json = json.loads(result)

    recognized_words = [word["word"] for word in result_json.get("result", [])]
    return jsonify({"recognized_words": recognized_words})


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5000)
