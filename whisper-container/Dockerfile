FROM python:3.12

COPY requirements.txt .
RUN wget https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz && tar xvf ffmpeg-release-amd64-static.tar.xz && mv ffmpeg-*-amd64-static/ffmpeg /usr/bin/ffmpeg && rm -Rf ffmpeg*

RUN pip install --no-cache-dir -r requirements.txt

ENTRYPOINT ["whisper"]
