from flask import Flask, Blueprint, jsonify, request
from interface import User
import pymongo, os, sys, time, uuid, json, utils, process

from auth import auth
from info import info
from user import user

app = Flask(__name__)
app.register_blueprint(auth)
app.register_blueprint(info)
app.register_blueprint(user)

BACKEND_VERSION="v0.0.1"

@app.route('/')
def index():
    return 'Welcome to N2Backend '+BACKEND_VERSION

if __name__ == '__main__':
    app.debug = False
    app.run(host="0.0.0.0", port=8443)