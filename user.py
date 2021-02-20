from flask import Flask, Blueprint, jsonify, request, make_response
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process

try:
    config=utils.read_json_file("config.json")
except:
    sys.exit("json format fail")

DB_ADDRESS=config["db_address"]

try:
    cli = process.connect_server(DB_ADDRESS)
except BaseException as e:    # Not work, solve WIP
    print(e)
    os._exit(0)

process.check_db(cli)

db=cli["N2-Station"]
users=db['users']
rooms=db['rooms']
tokens=db['tokens']

'''
“流名称”:{
    “title”: “房间标题”,
    “desc”: “房间描述”,
    “image”: ”图片url”,
    “tag”: [“tag的内容”, "...",],
    “status”: “几个状态 暂定open / close”,
    “user”: {
        “id”: “用户uuid”,
        ”name”: “用户名”
    }
}
'''

user = Blueprint('user', __name__)

@user.route('/api/user/rooms', methods=["GET"])
def getUserRoomList():
    try:
        user=process.get_user_by_token(db,request.cookies.get('Authorization'))
    except BaseException as e:
        return jsonify(utils.simple_reply("getUserRoomList",str(e)))

    data={ "data":utils.delete_key(user.rooms,["_id"]), "action": "getUserRoomList", "status":0 }
    #data["data"]["user"]={ "id": user.id, "user": user.name, "email": user.email}

    return jsonify(data)

@user.route('/api/user/createRoom', methods=["POST"])
def createRoom():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("createRoom", -11))

    try:
        user=process.get_user_by_token(db,request.cookies.get('Authorization'))
    except BaseException as e:
        return jsonify(utils.simple_reply("createRoom",str(e)))

    data_list=["id","title","desc","image","tag"]
    data={}
    for data_key in data_list:
        if data_key not in content:
            return jsonify(utils.simple_reply("createRoom", -10))
        data[data_key]=content[data_key]
    if data["id"] in user.rooms:
        return jsonify(utils.simple_reply("createRoom", -1))
    if len(user.rooms) >= user.limit["room_count"]:
        return jsonify(utils.simple_reply("createRoom", -2))

    # 创建
    try:
        user.create_room(data)
        return jsonify(utils.simple_reply("createRoom", 0))
    except:
        return jsonify(utils.simple_reply("createRoom",-3))

@user.route('/api/user/deleteRoom', methods=["POST"])
def deleteRoom():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("deleteRoom", -11))

    try:
        user=process.get_user_by_token(db,request.cookies.get('Authorization'))
    except BaseException as e:
        return jsonify(utils.simple_reply("deleteRoom",str(e)))

    if "id" not in content:
        return jsonify(utils.simple_reply("deleteRoom", -10))
    
    if content["id"] not in user.rooms:
        return jsonify(utils.simple_reply("deleteRoom", -1))

    # 删除
    try:
        user.delete_room(content)
        return jsonify(utils.simple_reply("deleteRoom", 0))
    except:
        return jsonify(utils.simple_reply("deleteRoom", -2))

@user.route('/api/user/openRoom', methods=["POST"])
def openRoom():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("openRoom", -11))

    try:
        user=process.get_user_by_token(db,request.cookies.get('Authorization'))
    except BaseException as e:
        return jsonify(utils.simple_reply("openRoom",str(e)))

    if "id" not in content:
        return jsonify(utils.simple_reply("openRoom", -10))

    if content["id"] in user.streaming:
        return jsonify(utils.simple_reply("openRoom", 0))
    
    if content["id"] not in user.rooms:
        return jsonify(utils.simple_reply("openRoom", -1))

    if len(user.streaming)>=user.limit["stream_count"]:
        return jsonify(utils.simple_reply("openRoom", -2))

    #启动
    try:
        user.open_room(content)
        return jsonify(utils.simple_reply("openRoom", 0))
    except BaseException as e:
        return jsonify(utils.simple_reply("openRoom", -3, e))
    

@user.route('/api/user/closeRoom', methods=["POST"])
def closeRoom():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("closeRoom", -2))

    try:
        user=process.get_user_by_token(db,request.cookies.get('Authorization'))
    except BaseException as e:
        return jsonify(utils.simple_reply("closeRoom",str(e)))

    if "id" not in content:
        return jsonify(utils.simple_reply("closeRoom", -10))

    if content["id"] not in user.streaming:
        return jsonify(utils.simple_reply("closeRoom", -1))
    
    if content["id"] not in user.rooms:
        return jsonify(utils.simple_reply("closeRoom", -1))

    #关闭
    try:
        user.close_room(content)
        return jsonify(utils.simple_reply("closeRoom", 0))
    except:
        return jsonify(utils.simple_reply("closeRoom", -2))

# @user.route('/api/user/debug/<func>')
# def debug(func):

#     return make_response("API Unavailable"), 404

#     try:
#         content=json.loads(request.get_data())
#     except:
#         return jsonify(utils.simple_reply("debug", -11))

#     if func=="addUser":
#         try:
#             process.add_user_to_db(db,{})
#         except BaseException as e:
#             return str(e)
#         return "0"
#     if func=="delUser":
#         try:
#             data, code = process.delete_user_from_db(db,content)
#         except BaseException as e:
#             return str(e)
#         return data