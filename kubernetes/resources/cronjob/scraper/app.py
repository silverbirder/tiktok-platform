from TikTokApi import TikTokApi

api = TikTokApi.get_instance()
device_id = api.generate_device_id()
tiktoks = api.by_username(
    'mi_channel.tiktok', 
    count=1, 
    custom_device_id=device_id,
    custom_verifyFp='verify_kvi3cgoo_RKTqODCc_VcxU_4Rd3_8xw1_uq1NCU55HSv9'
)

print(tiktoks)