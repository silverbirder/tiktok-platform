from flask import Flask, jsonify, request

app = Flask(__name__)

@app.route("/api/by_username")
def by_username():
    from TikTokApi import TikTokApi
    api = TikTokApi.get_instance()
    device_id = api.generate_device_id()
    params = request.args
    
    tiktoks = api.by_username(
        params.get('user_name'), 
        count=int(params.get('count', '1')), 
        custom_device_id=device_id,
        custom_verifyFp=params.get('custom_verifyFp', '')
    )
    return jsonify(tiktoks)

if __name__ == "__main__":
    app.run(host='0.0.0.0')