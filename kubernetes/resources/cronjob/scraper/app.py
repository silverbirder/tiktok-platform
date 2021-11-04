import os
import json

from TikTokApi import TikTokApi
from dotenv import load_dotenv

load_dotenv()

api = TikTokApi.get_instance()
device_id = api.generate_device_id()
tiktoks = api.by_username(
    os.environ.get('USER_NAME', 'mi_channel.tiktok'),
    count=int(os.environ.get('COUNT', '1')),
    custom_device_id=device_id,
    custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
)

print(json.dumps(tiktoks))