from flask import Flask, jsonify
import pymongo, os, sys, time, utils, process

app = Flask(__name__)

cache={}

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

DB_ADDRESS="mongodb://localhost:27017/"

try:
    db = process.connect_server(DB_ADDRESS)
except BaseException as e:
    print(e)
    os._exit(0)


users=db['users']

@app.route('/')
def index():
    return 'hello man'


'''
{
	"id": "uuid",
        "name": "username",
        "pass": "md5(this.name+md5(password))",
        "email":"email",
        "rooms": [],
        "limit": {
            "room_count": 5
        }
}
'''
@app.route('/api/info/user', methods=["GET"])
def getUserList():
    if "userList" not in cache:
        cache["userList"]={}
        for i in users.find():
            cache["userList"][str(i["_id"])]={ "id": str(i["_id"]), "name": i["name"], "email": i["email"] }
    return jsonify(cache["userList"])

@app.route('/api/user/debug/<func>')
def debug(func):
    if func=="addUser":
        try:
            global cache
            utils.add_test_user_to_db(db,{},cache)
        except BaseException as e:
            return str(e)
        return "0"

if __name__ == '__main__':
    app.debug = False
    app.run()