from flask import Flask, Blueprint, jsonify, request
from flask_sockets import Sockets
from flask_cors import CORS
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process, cron

from gevent import pywsgi
from geventwebsocket.handler import WebSocketHandler

from flask_limiter import Limiter
from flask_limiter.util import get_remote_address

from auth import auth
from info import info
from user import user
from ws import ws

try:
    config=utils.read_json_file("config.json")
except BaseException as e:
    print(e)
    sys.exit("json format fail")

app = Flask(__name__)
limiter = Limiter(
    app,
    key_func=get_remote_address,
    default_limits=["40 per minute", "500 per hour"]
)
app.register_blueprint(auth)
app.register_blueprint(info)
app.register_blueprint(user)

sockets = Sockets(app)
sockets.register_blueprint(ws)

CORS(app, send_wildcard=True, supports_credentials=True, origins=["https://live.4g.cx","https://n2station.live","http://dev.n2station.live:20606","http://localhost:8080"])

BACKEND_VERSION="v0.0.1"

@app.route('/')
def index():
    return 'Welcome to N2Backend '+BACKEND_VERSION

if __name__ == '__main__':

    cron.run()
    
    if 'ssl_cert_path' in config and 'ssl_key_path' in config:
        # app.run(host="0.0.0.0", port=8443, ssl_context=(config['ssl_cert_path'], config['ssl_key_path']))
        server = pywsgi.WSGIServer(('', 8443), app, handler_class=WebSocketHandler, certfile=config['ssl_cert_path'], keyfile=config['ssl_key_path'])
    else:
        server = pywsgi.WSGIServer(('', 8443), app, handler_class=WebSocketHandler)
    
    server.serve_forever()