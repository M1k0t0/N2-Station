# from gevent import monkey
# monkey.patch_all()
from flask import Flask, Blueprint, request
from flask_sockets import Sockets
from gevent import Timeout
from interface import User
import time, process, uuid, json, sys, os, utils

ws = Blueprint('ws', __name__)

pool={}

add_client_callback=None
del_client_callback=None

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
rooms=db['rooms']

def add_client(room,client):
    client_id=str(uuid.uuid4())[0:8]
    if room not in pool:
        pool[room]={}
    pool[room][client_id]=client
    if add_client_callback:
        add_client_callback.send(room,'| client',client_id,'join')
    return client_id

def del_client(room,client_id):
    if room in pool and client_id in pool[room]:
        del pool[room][client_id]
    if del_client_callback:
        del_client_callback.send(room,'| client',client_id,'leave')

def auth(token):
    return process.get_user_by_token(db,token)

def resolve(recv):
    try:
        space_index=recv.index(' ')
        func=recv[:space_index]
        data=recv[space_index+1:]
        return (func,data)
    except:
        return ('error','-11')

def broadcast(room,msg):
    for client in pool[room].values():
        client.send(msg)

@ws.route('/ping')
def ping(socket):
    add_client('ping',socket)
    while not socket.closed:
        socket.send('ping')
        with Timeout(1, False):
            socket.receive()
    del_client('ping',socket)

@ws.route('/api/chat/<room>')
def chat(socket,room):
    roomObj=rooms.find_one({"_id":room})
    if roomObj==None:
        socket.close()
        return

    client_id=add_client(room,socket)

    broadcast(room,"members "+str(len(pool[room])))

    guestMode=True

    token=request.cookies.get('Authorization')
    if token:
        try:
            user=auth(token)
        except BaseException as e:
            socket.send(str(e))
        else:
            guestMode=False
            chat_head='chat '+user.name+';'
            socket.send("auth ok")
            broadcast(room,'join '+user.name+";"+str(len(pool[room])))
    else:
        broadcast(room,"join Guest"+client_id+";"+str(len(pool[room])))

    while not socket.closed:
        recv=None
        try:
            with Timeout(15, False): # seems like Timeout() has bug
                recv=socket.receive()
            if recv==None:
                raise('Timeout or Connection closed')
        except:
            socket.close()
        else:
            if recv=='PING_PACK':
                continue
            
            func,data=resolve(recv)

            if func=="error":
                socket.send(data)

            if func=="auth":
                if data!='' and data!="GUEST":
                    try:
                        user=auth(data)
                    except BaseException as e:
                        socket.send(str(e))
                    else:
                        guestMode=False
                        chat_head='chat '+user.name+';'
                        socket.send("auth ok")

            if func=="message" and not guestMode:
                broadcast(room,chat_head+data.replace('\n',''))
    
    del_client(room,client_id)
    if guestMode:
        broadcast(room,"leave Guest"+client_id+";"+str(len(pool[room])))
    else:
        broadcast(room,'leave '+user.name+";"+str(len(pool[room])))