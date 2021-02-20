from flask import Flask, Blueprint, jsonify, request
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process

try:
    config=utils.read_json_file("config.json")
except BaseException as e:
    print(e)
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

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

RETURN_DATA=0
RETURN_CODE=-1

'''
{
	"id": "uuid",
    "name": "username",
    "pass": "utils.get_hashed_password(password)",
    "email":"email",
    "rooms": {
        "流名称": {
            “title”: “房间标题”,
            “desc”: “房间描述”,
            “image”: ”图片url”,
            “tag”: [“tag的内容”, "...",],
            “status”: “几个状态 暂定open / close”
        },
        “流名称”: { ... },
    },
    "streaming": {
        "流名称": {
            "time": ""
        }
    }
    "limit": {
        "room_count": 5,
        "stream_count": 1
    },
    token:{
        str(uuid.uuid4()): {
            time: time.time(),
            permission: -1
        }
    }
}
'''

auth = Blueprint('auth', __name__)

@auth.route('/api/auth/register', methods=["POST"])
def register():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("register", -11))
    if "name" not in content or "email" not in content or "pass" not in content or "code" not in content:
        return jsonify(utils.simple_reply("register", -10))
    validCode=db["invite_code"].find_one({'code':content["code"]})
    if validCode==None:
        return jsonify(utils.simple_reply("register", -4))

    try:
        data, code=process.add_user_to_db(db,{
            "email": content["email"].lower(), 
            "name": content["name"], 
            "password": content["pass"], 
            "rooms": {}, 
            "streaming":[],
            "limit": { "room_count":5, "stream_count": 1 }, 
            "token": {},
        })
    except BaseException as e:
        return jsonify(utils.simple_reply("register",-4,e))
    if code<0:
        return jsonify(utils.simple_reply("register",data))

    try:
        user=User(db,{"id":data})
    except BaseException as e:
        return jsonify({ "action": "register", "status": 0, "id": data })
    response=jsonify({ "action": "register", "status": 0, "id": data })
    response.set_cookie('Authorization',user.generate_token(),expires=utils.get_date_after(90))
    db["invite_code"].delete_one({'code':content["code"]})
    return response

@auth.route('/api/auth/getToken', methods=["POST"])
def getToken():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("getToken", -11))

    try:
        user=User(db, content)
    except BaseException as e:
        return jsonify(utils.simple_reply("getToken",str(e)))
    
    if "pass" not in content:
        return jsonify(utils.simple_reply("getToken", -10))

    if utils.check_password(content["pass"], user.password):
        response=jsonify(utils.simple_reply("getToken",0))
        response.set_cookie('Authorization',user.generate_token(),expires=utils.get_date_after(90))
        return response
    return jsonify(utils.simple_reply("getToken",-1))

@auth.route('/api/auth/destroyToken', methods=["GET","POST"])
def destroyToken():
    token=request.cookies.get('Authorization')
    if token == None:
        return jsonify(utils.simple_reply("destroyToken",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("destroyToken",-1))

    try:
        user=User(db,{"id":t["userID"]})
    except BaseException as e:
        return jsonify(utils.simple_reply("destroyToken",str(e)))
    
    if token not in user.token:
        return jsonify(utils.simple_reply("destroyToken",-1))

    del user.token[token]
    user.sync_data()

    response=jsonify(utils.simple_reply("destroyToken",0))
    response.delete_cookie('Authorization')
    return response

@auth.route('/api/auth/changePassword', methods=["POST"])
def changePassword():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("destroyToken", -11))
    
    token=request.cookies.get('Authorization')
    if token==None:
        return jsonify(utils.simple_reply("destroyToken",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("destroyToken",-2))
    
    try:
        user=User(db,{"id":t["userID"]})
    except BaseException as e:
        return jsonify(utils.simple_reply("destroyToken",str(e)))

    if "oldPass" not in content or "newPass" not in content:
        return jsonify(utils.simple_reply("destroyToken",-10))
    
    oldPass=content["oldPass"]
    newPass=content["newPass"]
    # 强度校验WIP
    if not utils.check_password(oldPass, user.password):
        return jsonify(utils.simple_reply("destroyToken",-1))

    user.password=utils.get_hashed_password(newPass)
    user.sync_data()


@auth.route('/api/auth/callback', methods=["GET"])
def verify():
    d = {
        "email": request.values.get("email"),
        "name": request.values.get("user"),
        "id": request.values.get("id")
    }

    method, data = utils.get_input(d)
        
    if method==None:
        return jsonify(utils.simple_reply("verify",-10)), 403

    method=method.replace("id","_id",1)
    user=users.find_one({method:data})
    if user != None:
        if utils.check_password(request.values.get("pass"), user["password"]):
            return jsonify(utils.simple_reply("verify",0))
    return jsonify(utils.simple_reply("verify",-1)), 403