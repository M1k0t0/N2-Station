from flask import Flask, Blueprint, jsonify, request
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

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

RETURN_DATA=0
RETURN_CODE=-1

info = Blueprint('info', __name__)

@info.route('/api/info/room', methods=["GET","POST"])
def getRoomList():
    if request.method=="GET":
        roomList={}
        tmp=rooms.find()
        for i in tmp:
            roomList[str(i["_id"])]=i
            try:
                user=User(db,{"id": i["userID"]})
            except:
                return jsonify(utils.simple_reply("getRoomList",-11))
            roomList[str(i["_id"])]["user"]={ "id": user.id, "name": user.name, "email": user.email }
            del roomList[str(i["_id"])]["userID"]
            del roomList[str(i["_id"])]["_id"]
        return jsonify({ "data": roomList, "action": "getRoomList", "status": 0 })
    elif request.method=="POST":
        try:
            content=json.loads(request.get_data())
        except:
            return jsonify(utils.simple_reply("searchRoom", -11))
        if "id" not in content:
            return jsonify(utils.simple_reply("searchRoom", -10))
        i=rooms.find_one({ "_id": content["id"] })
        if i==None:
            return jsonify(utils.simple_reply("searchRoom",-1))
        try:
            user=User(db,{"id": i["userID"]})
        except:
            return jsonify(utils.simple_reply("searchRoom",-11))
        room=utils.delete_key(i,["_id","userID"])
        room["user"]={ "id": user.id, "name": user.name, "email": user.email }
        room["action"]="searchRoom"
        room["status"]=0
        return jsonify(room)

@info.route('/api/info/user', methods=["GET","POST"])
def getUserList():
    if request.method=="GET":
        userList={}
        for i in users.find():
            userList[str(i["_id"])]={ "id": str(i["_id"]), "name": i["name"], "email": i["email"] }
        return jsonify({ "data": userList, "action": "getUserList", "status": 0 })
    elif request.method=="POST":
        try:
            content=json.loads(request.get_data())
        except:
            return jsonify(utils.simple_reply("searchUser", -11))
        if "id" not in content and "name" not in content and "email" not in content:
            return jsonify(utils.simple_reply("searchUser", -10))
        try:
            user=User(db, content)
        except BaseException as e:
            return jsonify(utils.simple_reply("searchUser",str(e)))
        return jsonify({ "id": user.id, "name": user.name, "email": user.email, "action": "searchUser", "status": 0 })