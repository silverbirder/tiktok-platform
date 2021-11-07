import os
import json

from TikTokApi import TikTokApi
from dotenv import load_dotenv

load_dotenv()

api = TikTokApi.get_instance()
device_id = api.generate_device_id()

pager = api.get_user_pager(
    os.environ.get('USER_NAME', ''), 
    page_size=5,
    custom_device_id=device_id,
    custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
)

for post in pager:    
    print(post)

api.clean_up()