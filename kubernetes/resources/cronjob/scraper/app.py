import os
import json

from TikTokApi import TikTokApi
from dotenv import load_dotenv

load_dotenv()

print(json.dumps('{"id": "7032969809184738562", "desc": "\u4e2d\u5b66\u751f\u306e\u9803\u53cb\u9054\u304c\u30ae\u30e3\u30eb\u30b5\u30fc\u5165\u3063\u3066\u305f\ud83d\udc71\u200d\u2640\ufe0f #\u307f\u30fc\u3061\u3083\u3093\u306d\u308b #\u3080\u3061\u3080\u3061 #\u30cb\u30c3\u30c8\u5973\u5b50 #\u30d4\u30c1\u30d4\u30c1", "createTime": 1637490889, "video": {"id": "7032969809184738562", "height": 1024, "width": 576, "duration": 13, "ratio": "720p", "cover": "https://p16-sign-sg.tiktokcdn.com/obj/tos-alisg-p-0037/843805d94f3549f7ac7884a625b3af65?x-expires=1637564400&x-signature=cNuIu2PuOQJzqsQ1pqY9yQwlr40%3D", "originCover": "https://p16-sign-sg.tiktokcdn.com/obj/tos-alisg-p-0037/cf8e93efa0bb46afa8ae1d018def31e4_1637490889?x-expires=1637564400&x-signature=ufD6p62zcpSk%2FL5hWvyvrC%2BGs3g%3D", "dynamicCover": "https://p16-sign-sg.tiktokcdn.com/obj/tos-alisg-p-0037/2496fe425e374bebba5cf6ba8a0aad21_1637490890?x-expires=1637564400&x-signature=5Q7zbIwaD38M%2Bg6t%2BClCIGfTkCA%3D", "playAddr": "https://v16-web.tiktok.com/video/tos/alisg/tos-alisg-pve-0037/547c6127634f4ce79a47443593a8381e/?a=1988&br=2274&bt=1137&cd=0%7C0%7C1&ch=0&cr=0&cs=0&cv=1&dr=3&ds=3&er=&expire=1637567073&ft=wUyFfFM5kag3-I&l=20211122014420010223081157239569AB&lr=tiktok&mime_type=video_mp4&net=0&pl=0&policy=3&qs=0&rc=M2U4Njk6Zmo1OTMzODgzNEApNDY7ZTVpNDw7N2VoODg5ZWdvMW9ucjRfLmFgLS1kLy1zc2EuNTA1LjQwMDIwYzQzYC46Yw%3D%3D&signature=5c8dbea46a6cdd50638bf9d6a5cce7af&tk=0&vl=&vr=", "downloadAddr": "https://res.cloudinary.com/silverbirder/image/upload/w_1000,ar_16:9,c_fill,g_auto,e_sharpen/v1613137981/silver-birder.github.io/assets/IMG_3040.jpg", "shareCover": ["", "https://p16-sign-sg.tiktokcdn.com/tos-alisg-p-0037/cf8e93efa0bb46afa8ae1d018def31e4_1637490889~tplv-tiktok-play.jpeg?x-expires=1637564400&x-signature=bVY55juKH8YS48Q9aX1mavikjRY%3D", "https://p16-sign-sg.tiktokcdn.com/tos-alisg-p-0037/cf8e93efa0bb46afa8ae1d018def31e4_1637490889~tplv-tiktokx-share-play.jpeg?x-expires=1637564400&x-signature=h3uxE51VJSLcqxL4kSG3OpNBuvQ%3D"], "reflowCover": "https://p16-sign-sg.tiktokcdn.com/obj/tos-alisg-p-0037/843805d94f3549f7ac7884a625b3af65?x-expires=1637564400&x-signature=cNuIu2PuOQJzqsQ1pqY9yQwlr40%3D", "bitrate": 1164998, "encodedType": "normal", "format": "mp4", "videoQuality": "normal", "encodeUserTag": "", "codecType": "h264", "definition": "720p"}, "author": {"id": "6586988714068393986", "uniqueId": "mi_channel.tiktok", "nickname": "\u307f\u30fc\u3061\u3083\u3093\u306d\u308b\u2763\ufe0f", "avatarThumb": "https://p77-sign-sg.tiktokcdn.com/aweme/100x100/tos-alisg-avt-0068/80f751e4fb0c669899ef0aab6c33b90e.jpeg?x-expires=1637629200&x-signature=l1Mz9Sk%2BXC1dMDSrV2Rzve0I9LU%3D", "avatarMedium": "https://p16-sign-sg.tiktokcdn.com/aweme/720x720/tos-alisg-avt-0068/80f751e4fb0c669899ef0aab6c33b90e.jpeg?x-expires=1637629200&x-signature=xOKvSQrvJhMqEexq4f3lL4b4wJU%3D", "avatarLarger": "https://p77-sign-sg.tiktokcdn.com/aweme/1080x1080/tos-alisg-avt-0068/80f751e4fb0c669899ef0aab6c33b90e.jpeg?x-expires=1637629200&x-signature=K%2B4XotE0KO81KHYSkMMsx%2BTyo9E%3D", "signature": "Twitter\u3068candfans\u30e1\u30a4\u30f3\u3067\u3059\ud83d\udc95\n\ud83d\udc47\u304b\u3089\u898b\u305f\u3044\u3068\u3053\u308d\u306b\u98db\u3093\u3067\u306d\ud83e\udd70", "verified": false, "secUid": "MS4wLjABAAAArfugzxmXF5Os0y45fZfOhCoMEux5TWmpQkSrvyBWdoMe0Mkjcn3PFpW0nrqbn10F", "secret": false, "ftc": false, "relation": 0, "openFavorite": false, "commentSetting": 0, "duetSetting": 0, "stitchSetting": 0, "privateAccount": false}, "music": {"id": "6832122401436388097", "title": "\u30aa\u30ea\u30b8\u30ca\u30eb\u697d\u66f2 - \u30b9\u30df\u30ec", "playUrl": "https://sf16-ies-music-sg.tiktokcdn.com/obj/tiktok-obj/6889894239782587138.mp3", "coverThumb": "https://p16-sign-sg.tiktokcdn.com/aweme/100x100/tos-alisg-avt-0068/2c2d4d5148b071bb87a6632b67599e65.jpeg?x-expires=1637629200&x-signature=nwDG2tDOXt9HJBZpFiXwtuyDHGY%3D", "coverMedium": "https://p16-sign-sg.tiktokcdn.com/aweme/720x720/tos-alisg-avt-0068/2c2d4d5148b071bb87a6632b67599e65.jpeg?x-expires=1637629200&x-signature=qTu2zokQSOJLERAS31aqp34siRY%3D", "coverLarge": "https://p16-sign-sg.tiktokcdn.com/aweme/1080x1080/tos-alisg-avt-0068/2c2d4d5148b071bb87a6632b67599e65.jpeg?x-expires=1637629200&x-signature=zm510RZxcnuWhg3CE%2FJvRo636Kw%3D", "authorName": "\u304a\u306b\u3061\u3083\u3093\u30c3\uff01", "original": true, "duration": 13, "album": ""}, "challenges": [{"id": "1641202588048385", "title": "\u307f\u30fc\u3061\u3083\u3093\u306d\u308b", "desc": "", "profileThumb": "", "profileMedium": "", "profileLarger": "", "coverThumb": "", "coverMedium": "", "coverLarger": "", "isCommerce": false}, {"id": "72134935", "title": "\u3080\u3061\u3080\u3061", "desc": "", "profileThumb": "", "profileMedium": "", "profileLarger": "", "coverThumb": "", "coverMedium": "", "coverLarger": "", "isCommerce": false}, {"id": "1619356548135938", "title": "\u30cb\u30c3\u30c8\u5973\u5b50", "desc": "", "profileThumb": "", "profileMedium": "", "profileLarger": "", "coverThumb": "", "coverMedium": "", "coverLarger": "", "isCommerce": false}, {"id": "1603144708747266", "title": "\u30d4\u30c1\u30d4\u30c1", "desc": "", "profileThumb": "", "profileMedium": "", "profileLarger": "", "coverThumb": "", "coverMedium": "", "coverLarger": "", "isCommerce": false}], "stats": {"diggCount": 538, "shareCount": 9, "commentCount": 24, "playCount": 6791}, "duetInfo": {"duetFromId": "0"}, "originalItem": false, "officalItem": false, "textExtra": [{"awemeId": "", "start": 23, "end": 31, "hashtagName": "\u307f\u30fc\u3061\u3083\u3093\u306d\u308b", "hashtagId": "1641202588048385", "type": 1, "userId": "", "isCommerce": false, "userUniqueId": "", "secUid": "", "subType": 0}, {"awemeId": "", "start": 32, "end": 37, "hashtagName": "\u3080\u3061\u3080\u3061", "hashtagId": "72134935", "type": 1, "userId": "", "isCommerce": false, "userUniqueId": "", "secUid": "", "subType": 0}, {"awemeId": "", "start": 38, "end": 44, "hashtagName": "\u30cb\u30c3\u30c8\u5973\u5b50", "hashtagId": "1619356548135938", "type": 1, "userId": "", "isCommerce": false, "userUniqueId": "", "secUid": "", "subType": 0}, {"awemeId": "", "start": 45, "end": 50, "hashtagName": "\u30d4\u30c1\u30d4\u30c1", "hashtagId": "1603144708747266", "type": 1, "userId": "", "isCommerce": false, "userUniqueId": "", "secUid": "", "subType": 0}], "secret": false, "forFriend": false, "digged": false, "itemCommentStatus": 0, "showNotPass": false, "vl1": false, "itemMute": false, "authorStats": {"followingCount": 60, "followerCount": 24500, "heartCount": 106400, "videoCount": 57, "diggCount": 63, "heart": 106400}, "privateItem": false, "duetEnabled": true, "stitchEnabled": true, "shareEnabled": true, "stickersOnItem": [{"stickerType": 4, "stickerText": ["\u30e9\u30d6\u30b8\u30e7\u30a4\u3068\u3044\u3048\u3070", "\u30d1\u30e9\u30d1\u30e9\u6d3e\ud83e\udd70"]}], "isAd": false, "duetDisplay": 0, "stitchDisplay": 0}'))

# api = TikTokApi.get_instance()
# device_id = api.generate_device_id()
# tiktoks = api.by_username(
#     os.environ.get('USER_NAME', 'mi_channel.tiktok'),
#     count=int(os.environ.get('COUNT', '1')),
#     custom_device_id=device_id,
#     custom_verifyFp=os.environ.get('CUSTOM_VERIFY_FP', '')
# )

# print(json.dumps(tiktoks))

# api.clean_up()