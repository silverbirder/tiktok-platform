import os
import json

from TikTokApi import TikTokApi
from dotenv import load_dotenv

load_dotenv()

api = TikTokApi.get_instance()
device_id = api.generate_device_id()
user = api.get_user(
    username=os.environ.get('USER_NAME', ''),
    custom_device_id=device_id,
    custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
)

print(json.dumps(user))

api.clean_up()