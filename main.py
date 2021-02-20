from flask import Flask, Blueprint, jsonify, request
from flask_cors import CORS
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process

from auth import auth
from info import info
from user import user

try:
    config=utils.read_json_file("config.json")
except BaseException as e:
    print(e)
    sys.exit("json format fail")

app = Flask(__name__)
app.register_blueprint(auth)
app.register_blueprint(info)
app.register_blueprint(user)

CORS(app, send_wildcard=True, supports_credentials=True, origins=["https://live.4g.cx","https://n2station.live","http://localhost:8080"])

BACKEND_VERSION="v0.0.1"

@app.route('/')
def index():
    return 'Welcome to N2Backend '+BACKEND_VERSION

if __name__ == '__main__':
    app.debug = False
    if 'ssl_cert_path' in config and 'ssl_key_path' in config:
        app.run(host="0.0.0.0", port=8443, ssl_context=(config['ssl_cert_path'], config['ssl_key_path']))
    else:
        app.run(host="0.0.0.0", port=8443)