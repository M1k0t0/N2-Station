from flask import Flask, jsonify, request
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process

app = Flask(__name__)

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

RETURN_DATA=0
RETURN_CODE=-1

DB_ADDRESS="mongodb://localhost:27017/"

BACKEND_VERSION="v0.0.1"

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

@app.route('/')
def index():
    return 'Welcome to N2Backend '+BACKEND_VERSION

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

@app.route('/api/user/createRoom', methods=["POST"])
def createRoom():
    content=json.loads(request.get_data())

    token=request.cookies.get('Authorization')
    if token==None:
        return jsonify(utils.simple_reply("createRoom",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("createRoom",-3))
    
    try:
        user=User(db,{"id":t["userID"]})
        user.sync_data()
    except BaseException as e:
        return jsonify(utils.simple_reply("createRoom",str(e)))

    data_list=["id","title","desc","image","tag"]
    data={}
    for data_key in data_list:
        if data_key not in content:
            return jsonify(utils.simple_reply("createRoom", -10))
        data[data_key]=content[data_key]
    
    # 创建
    try:
        user.create_room(data)
        return jsonify(utils.simple_reply("createRoom", 0))
    except:
        return jsonify(utils.simple_reply("createRoom",-3))

@app.route('/api/user/deleteRoom', methods=["POST"])
def deleteRoom():
    content=json.loads(request.get_data())

    token=request.cookies.get('Authorization')
    if token==None:
        return jsonify(utils.simple_reply("deleteRoom",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("deleteRoom",-2))
    
    try:
        user=User(db,{"id":t["userID"]})
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

@app.route('/api/user/openRoom', methods=["POST"])
def openRoom():
    content=json.loads(request.get_data())

    token=request.cookies.get('Authorization')
    if token==None:
        return jsonify(utils.simple_reply("openRoom",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("openRoom",-3))
    
    try:
        user=User(db,{"id":t["userID"]})
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
    except:
        return jsonify(utils.simple_reply("openRoom", -3))

@app.route('/api/user/closeRoom', methods=["POST"])
def closeRoom():
    content=json.loads(request.get_data())

    token=request.cookies.get('Authorization')
    if token==None:
        return jsonify(utils.simple_reply("closeRoom",-10))
    t=db["tokens"].find_one({"token": token})
    if t == None:
        return jsonify(utils.simple_reply("closeRoom",-2))
    
    try:
        user=User(db,{"id":t["userID"]})
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
    "limit": {
        "room_count": 5,
        "stream_count": 1
    },
    token:{
        str(uuid.uuid1()): {
            time: time.time(),
            permission: -1
        }
    }
}
'''
@app.route('/api/info/user', methods=["GET"])
def getUserList():
    userList={}
    for i in users.find():
        userList[str(i["_id"])]={ "id": str(i["_id"]), "user": i["user"], "email": i["email"] }
    return jsonify({ "data": userList, "action": "getUserList" })

@app.route('/api/user/debug/<func>')
def debug(func):
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("debug", -11))

    if func=="addUser":
        try:
            process.add_user_to_db(db,{})
        except BaseException as e:
            return str(e)
        return "0"
    if func=="delUser":
        try:
            data, code = process.delete_user_from_db(db,content)
        except BaseException as e:
            return str(e)
        return data

@app.route('/api/auth/register', methods=["POST"])
def register():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("register", -11))
    if "user" not in content or "email" not in content or "pass" not in content:
        return jsonify(utils.simple_reply("register", -10))
    try:
        data, code=process.add_user_to_db(db,{
            "email": content["email"], 
            "user": content["user"], 
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
    return response

@app.route('/api/auth/getToken', methods=["POST"])
def getToken():
    try:
        content=json.loads(request.get_data())
    except:
        return jsonify(utils.simple_reply("getToken", -11))

    try:
        user=User(db, content)
    except BaseException as e:
        return jsonify(utils.simple_reply("getToken",str(e))), 403
    
    if "pass" not in content:
        return jsonify(utils.simple_reply("getToken", -10))

    if utils.check_password(content["pass"], user.password):
        response=jsonify(utils.simple_reply("getToken",0))
        response.set_cookie('Authorization',user.generate_token(),expires=utils.get_date_after(90))
        return response
    return jsonify(utils.simple_reply("getToken",-1)), 403

@app.route('/api/auth/destroyToken', methods=["GET","POST"])
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

@app.route('/api/auth/changePassword', methods=["POST"])
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


@app.route('/api/auth/callback', methods=["POST"])
def verify():
    d = {
        "email": request.values.get("email"),
        "user": request.values.get("user"),
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


if __name__ == '__main__':
    app.debug = False
    app.run(host="0.0.0.0", port=8443)