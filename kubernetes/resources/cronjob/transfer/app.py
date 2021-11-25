import os
import json
from urllib.parse import urlparse
import json

from TikTokApi import TikTokApi
from dotenv import load_dotenv
from google.cloud import storage
import requests

load_dotenv()

tiktok_json = json.loads(os.environ.get('INPUT_DATA', '{}'))

# Decide on a video file name.
download_url = tiktok_json.get('video', {}).get('downloadAddr', '')
format = tiktok_json.get('video', {}).get('format', 'mp4')
file_name_candidates = urlparse(download_url).path.split('/')
candidates_index = len(file_name_candidates)-1
file_name = 'temporary.{format}'.format(format=format)
while True:
    file_name_candidate = file_name_candidates[candidates_index]
    if file_name_candidate != '':
        file_name = '{name}.{format}'.format(name=file_name_candidate, format=format)
        break
    candidates_index-=1

# Download video
api = TikTokApi.get_instance()
device_id = api.generate_device_id()

video_bytes = api.get_video_by_download_url(
    download_url=download_url,
    custom_device_id=device_id,
    custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
)

api.clean_up()

# Save video
video_file_name = './video/{file_name}'.format(file_name=file_name)
f = open(video_file_name, 'wb')
f.write(video_bytes)
f.close()

# Upload video
client = storage.Client()
bucket_name = '{gcp_project}-bucket'.format(gcp_project=os.environ.get('GCP_PROJECT', ''))
bucket = client.get_bucket(bucket_name)
video_new_blob = bucket.blob('video/{file_name}'.format(file_name=file_name))
video_new_blob.upload_from_filename(filename=video_file_name)
tiktok_json['video']['downloadAddr'] = 'https://storage.googleapis.com/{bucket_name}/{file_name}'.format(bucket_name=bucket_name, file_name=file_name)

print(tiktok_json)