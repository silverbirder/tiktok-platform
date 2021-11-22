import os
import json
from urllib.parse import urlparse

from TikTokApi import TikTokApi
from dotenv import load_dotenv

load_dotenv()

# Decide on a video file name.
download_url = os.environ.get('DOWNLOAD_URL', '')
file_name_candidates = urlparse(download_url).path.split('/')
candidates_index = len(file_name_candidates)-1
file_name = 'temporary.mp4'
while True:
    file_name_candidate = file_name_candidates[candidates_index]
    if file_name_candidate != '':
        file_name = '{name}.mp4'.format(name=file_name_candidate)
        break
    candidates_index-=1


api = TikTokApi.get_instance()
device_id = api.generate_device_id()

video_bytes = api.get_video_by_download_url(
    download_url=os.environ.get('DOWNLOAD_URL', ''),
    custom_device_id=device_id,
    custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
)

with open(file_name, "wb") as out:
    out.write(video_bytes)
    api.clean_up()