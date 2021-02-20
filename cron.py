import threading, json, process, utils, os, sys, time
from interface import User

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
rooms=db['rooms']

def sync_room_status():
    data=utils.get_stat()
    stat={}
    for item in data['http-flv']['servers'][0]['applications'][0]['live']['streams']:
        stat[item["name"]]=item
    roomList=rooms.find()
    for room in roomList:
        id=room['_id']
        if room['status']=='open' and (id not in stat or not stat[id]["publishing"]):
            if int(time.time()) - room["time"]["openTime"]<=300:
                continue
            user=User(db,{"id":room["userID"]})
            user.close_room({"id":id})

def threads():
    while True:
        sync_room_status()
        time.sleep(600)

def run():
    t=threading.Thread(target=threads)
    t.start()