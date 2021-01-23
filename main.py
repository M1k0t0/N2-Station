from flask import Flask, jsonify, request
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
except BaseException as e:    # Not work, solve WIP
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
        "pass": "utils.get_hashed_password(password)",
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
    return jsonify({ "data": cache["userList"], "action": "getUserList" })

@app.route('/api/user/debug/<func>')
def debug(func):
    if func=="addUser":
        try:
            global cache
            process.add_user_to_db(db,{},cache)
        except BaseException as e:
            return str(e)
        return "0"

@app.route('/api/auth/register', methods=["POST"])
def register():
    email=request.values.get("email")
    username=request.values.get("user")
    password=request.values.get("pass")
    if email==None or username==None or password==None:
        return jsonify(utils.simple_reply("register", -10))
    try:
        global cache
        data, code=process.add_user_to_db(db,{"email": email, "user": username, "pass": password, "rooms": [], "limit": { "room_count":5 }},cache)
    except BaseException:
        return jsonify(utils.simple_reply("register",-4))
    if code<0:
        return jsonify(utils.simple_reply("register",data))
    return jsonify({ "action": "register", "status": "0", "token": "", "id": data })    # Token WIP
    
    
@app.route('/api/auth/verify', methods=["POST"])
def verify():
    method=''
    data=''
    
    email=request.values.get("email") 
    if email != None:
        data=request.values.get("email") 
        method='email'
    
    username=request.values.get("user")
    if username != None:
        method='user'
        data=username
    
    id=request.values.get("id")
    if id != None:
        method='id'
        data=id
        
    if method=='':
        return jsonify(utils.simple_reply("verify",-10)), 403

    user=users.find_one({method:data})
    if user != None:
        if utils.check_password(user["pass"],request.values.get("pass")):
            return jsonify(utils.simple_reply("verify",0))
    return jsonify(utils.simple_reply("verify",-1)), 403


if __name__ == '__main__':
    app.debug = False
    app.run(host="0.0.0.0", port=8443)