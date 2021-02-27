<template>
    <v-container fluid fill-height class="align-center justify-center">
        <v-row
            align="center"
            justify="center"
            class="fill-height"
        >
            <v-col sm="12" md="11" class="pb-1 fill-height">
                <v-card tile class="px-6 pt-3 on-phone-card-padding-bottom on-desktop-card-padding-bottom" height="95%">
                    <v-row
                        align="stretch"
                        justify="center"
                        style="flex-wrap: wrap;"
                    >
                        <v-col sm="12" md="8" class="col-me">
                            <v-row align="center">
                                <v-col cols="10" class="py-0">
                                    <p class="title mb-0">{{ room.title }}</p>
                                    <p class="caption">{{ room.desc }}</p>
                                </v-col>
                                <v-col cols="2" class="py-0 text-end">
                                    <v-avatar
                                    :color="online_total?'teal':'error'"
                                    size="26"
                                    >{{ online_total }}</v-avatar>
                                </v-col>
                            </v-row>
                            <!-- <video
                            class="pl-sm-12 pr-sm-12 pt-sm-0 pb-sm-0 no-outline"
                            width="100%"
                            height="auto"
                            id="videoElement" controls autoplay>
                                Your browser is too old which doesn't support HTML5 video.
                            </video> -->
                            <div 
                            class="pt-sm-0 pb-sm-0 no-outline"
                            id="videoFrame"
                            ></div>
                            <v-card flat>
                                <v-row
                                    align="center"
                                    justify="start"
                                >
                                    <v-col cols="2">
                                        <v-gravatar 
                                            v-if="room" 
                                            class="" 
                                            :size="55" 
                                            :email="$root.roomList[$route.params.id].user.email" 
                                            style="display:inline; border-radius: 60px;" 
                                        />
                                    </v-col>
                                    <v-col cols="10">
                                        <p 
                                            v-if="room" 
                                            style="display:inline;font-size: 25px;"
                                        >
                                            {{$root.roomList[$route.params.id].user.name}}
                                        </p>
                                    </v-col>
                                </v-row>
                            </v-card>
                        </v-col>
                        <v-spacer />
                        <v-col sm="12" md="4" class="col-me pb-0 on-phone-chat-box">
                            <v-card tile id="chatBox" style="height:100%;" class="d-flex on-phone-chat-box-flex-reverse on-desktop-chat-box-flex" color="#36393f">
                                <v-card 
                                    height="20%"
                                    class="d-flex on-phone-chat-box-flex-reverse on-desktop-chat-box-flex px-5 pt-5" 
                                    color="#36393f"
                                >
                                    <p class="body-2 mb-1" v-if="ws_state">
                                        聊天服务状态
                                        <font color="#dcddde">：{{ws_state}}</font>
                                    </p>
                                    <p class="body-2 mb-1" v-if="ws_state">
                                        登录状态
                                        <font color="#dcddde">：{{ login?"在线":"游客" }}</font>
                                    </p>
                                    <p class="body-2 mb-1">
                                        房间码率(Kbps)
                                        <font color="#dcddde">：{{ getLiveInfo.bitrate?getLiveInfo.bitrate/1000:"0" }}</font>
                                    </p>
                                </v-card>
                                <v-card 
                                    flat 
                                    id="chatContent" 
                                    height="60%" 
                                    class="d-flex on-phone-chat-box-flex-reverse on-desktop-chat-box-flex px-5 pb-5 pt-2" 
                                    color="#36393f"
                                    style="overflow-y: auto;flex: 1 1 1px;"
                                >
                                    <p 
                                        class="body-2 mb-1" 
                                        v-for="(item,index) in msg_list"
                                        :key="index+''"
                                        style=""
                                    >
                                        <font v-if="item.length && item[0]!='0'">{{item[0]}}</font>
                                        <font color="#dcddde" v-if="item.length && item[0]!='0'">：</font>
                                        <font color="#dcddde" v-if="item.length>1">{{item[1]}}</font>
                                    </p>
                                </v-card>
                                <v-card color="#36393f" height="20%">
                                    <v-row align="center" class="mx-auto">
                                    <v-col cols="9" class="on-phone-chat-box-textarea">
                                        <v-textarea 
                                            :rows="2"
                                            no-resize
                                            solo 
                                            clearable
                                            hide-details
                                            color="white" 
                                            background-color="#40444b"
                                            :disabled="!login"
                                            :label="login?'发个弹幕呗~':'速速登录plz'"
                                            class="no-outline mb-auto"
                                            v-model="chat_msg"
                                            @keyup.enter.exact="ws_send('message '+chat_msg);chat_msg=null;"
                                        ></v-textarea>
                                    </v-col>
                                    <v-spacer/>
                                    <v-col cols="3" class="on-phone-chat-box-textarea">
                                        <v-btn 
                                            :disabled="!login || !chat_msg" 
                                            icon 
                                            color="white"
                                            @click="ws_send('message '+chat_msg);chat_msg=null;"
                                        >
                                            <v-icon color="white">
                                                mdi-arrow-up-thick
                                            </v-icon>
                                        </v-btn>
                                    </v-col>
                                    </v-row>
                                </v-card>
                            </v-card>
                        </v-col>
                    </v-row>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script>

import flvjs from 'flv.js';
import DPlayer from 'dplayer';
import { clearInterval, setInterval } from 'timers';

export default {
    data: () => ({
        player: {},
        room: null,
        chat_msg: null,
        ws: null,
        login: false,
        msg_list: [],
        ws_state: '连接中',
        reconnect_timer: null,
        heartbeat_timer: null,
        getLiveInfo: {},
        bitrate_monitor: null,
        closeWs: false,
        wsurl_base: "wss://n2station.live:8443/api/chat/",
        online_total: 0
    }),
    methods: {
        setSource(id){
            this.player.source = this.global_.pullAddress+"/live?port="+this.global_.pullAddressRTMPPort+"&app=rtmp&stream="+id;
            this.$nextTick(() => this.pullVideo());

        },
        loadRoom(id){
            this.setSource(id);
            // this.setChatboxHeight('chatBox');
        },
        pullVideo(){
            if (this.player.source && flvjs.isSupported()) {
                var videoFrame = document.getElementById('videoFrame');
                var videoSource = this.player.source;
                var flvPlayer = null;
                this.$root.DPlayer = new DPlayer({
                    container: videoFrame,
                    video: {
                        url: 'live.flv',
                        type: 'customFlv',
                        live: true,
                        playbackSpeed: [1],
                        customType: {
                            customFlv: function (video) {
                                flvPlayer = flvjs.createPlayer({
                                    type: 'flv',
                                    url: videoSource
                                });
                                flvPlayer.attachMediaElement(video);
                                flvPlayer.load();
                            }
                        }
                    }
                });
                this.$root.DPlayer.play();
                this.$root.flvPlayer=flvPlayer;
            }
        },
        setChatboxHeight(id){
            this.$nextTick(() => {
                if(document.documentElement.clientWidth>=960){
                    document.getElementById(id).style.height=document.getElementById('videoFrame').clientHeight-24+'px';
                }else
                    document.getElementById(id).style.height=document.documentElement.clientHeight-document.getElementById('videoFrame').clientHeight-24+'px';
            })
        },
        initWebSocket(id){
            if(this.ws) return;
            const wsurl = this.wsurl_base+id;
            this.ws = new WebSocket(wsurl);
            this.ws.onmessage = this.on_message;
            this.ws.onopen = this.on_open;
            this.ws.onerror = this.on_error;
            this.ws.onclose = this.on_close;
        },
        on_open(){
            this.closeWs=false;
            clearInterval(this.reconnect_timer);
            this.reconnect_timer=null;
            this.ws_state='已连接';
            this.heartbeat_timer=setInterval(()=>this.ws_send('PING_PACK'),7000);
        },
        on_error(){
            this.ws_state='发生错误，尝试重连';
            clearInterval(this.heartbeat_timer);
            if(!this.reconnect_timer){
                this.ws.close();
                this.ws=null;
                this.initWebSocket(this.$route.params.id);
                this.reconnect_timer=setInterval(()=>{
                    this.ws_state='发生错误，尝试重连';
                    this.ws.close();
                    this.ws=null;
                    this.initWebSocket(this.$route.params.id);
                }, 7000);
            }
        },
        on_message(e){
            if(e.target.url!=this.wsurl_base+this.$route.params.id) return;
            const recv = e.data;
            if(recv=="auth ok") this.login=true;
            if(recv.length>=8 && recv.substring(0,4)=="chat"){ // at least 8 char: "chat 0;a"
                let username=recv.substring(5,recv.indexOf(';'));
                let message=recv.substring(recv.indexOf(';')+1);
                this.push_message([username,message]);
                // this.$root.DPlayer.danmaku.draw({  // not work
                //     text: message,
                //     color: '#fff',
                //     type: 'top',
                // })
            }
            if(recv.length>=9 && recv.substring(0,7)=="members")
                this.online_total=recv.substring(8);
            if(recv.length>=8 && recv.substring(0,4)=="join"){
                let username=recv.substring(5,recv.indexOf(';'));
                this.online_total=recv.substring(recv.indexOf(';')+1);
                this.push_message(["0",username+" 加入了房间"]);
            }
            if(recv.length>=9 && recv.substring(0,5)=="leave"){
                let username=recv.substring(6,recv.indexOf(';'));
                this.online_total=recv.substring(recv.indexOf(';')+1);
                this.push_message(["0",username+" 退出了房间"]);
            }
        },
        ws_send(data){
            this.ws.send(data);
        },
        on_close(e){
            if(e.target.url==this.wsurl_base+this.$route.params.id){
                if(!this.reconnect_timer && !this.closeWs){
                    this.ws_state='正在重新建立连接';
                    this.ws.close();
                    this.ws=null;
                    this.initWebSocket(this.$route.params.id);
                    this.reconnect_timer=setInterval(()=>{
                        this.ws_state='正在重新建立连接';
                        this.ws.close();
                        this.ws=null;
                        this.initWebSocket(this.$route.params.id);
                    }, 7000);
                }else if(this.closeWs){
                    clearInterval(this.heartbeat_timer);
                }
            }
            console.log('Room Chat Websocket',e,'Disconnected.');
        },
        push_message(msg){
            if(this.msg_list.length>=100) this.msg_list.splice(0,1);
            this.msg_list.push(msg);
            var ele = document.getElementById('chatContent');
            ele.scrollTop = ele.scrollHeight;
        }
    },
    created() {
        this.initWebSocket(this.$route.params.id);
        if(!this.bitrate_monitor){
            this.global_.request.getLiveInfo(this,this.$route.params.id).then(() => {
                this.getLiveInfo=this.$root.getLiveInfo;
            });
            this.bitrate_monitor=setInterval(()=>this.global_.request.getLiveInfo(this,this.$route.params.id).then(() => {
                this.getLiveInfo=this.$root.getLiveInfo;
            }),20000);
        }
    },
    mounted() {
        this.room=this.$root.roomList[this.$route.params.id];
        if(!this.room){
            this.global_.request.asyncGetRoomList(this).then(() => {
                this.room=this.$root.roomList[this.$route.params.id];
                this.loadRoom(this.$route.params.id);
                this.$set(this.$root.bread,1,{
                    text: this.room.title,
                    disabled: true,
                    href: '#/live/'+this.$route.params.id,
                });
            })
        }else{
            this.loadRoom(this.$route.params.id);
            this.$set(this.$root.bread,1,{
                text: this.room.title,
                disabled: true,
                href: '#/live/'+this.$route.params.id,
            });
        }
    },
    beforeRouteUpdate (to, from, next) {
        if(this.$root.DPlayer) this.$root.DPlayer.destroy();
        if(this.ws){
            this.closeWs=true;
            this.ws.close();
            this.ws=null;
        }
        if(this.reconnect_timer) clearInterval(this.reconnect_timer);
        if(this.heartbeat_timer) clearInterval(this.heartbeat_timer);
        if(this.bitrate_monitor) clearInterval(this.bitrate_monitor);
        if(to.path.substring(0,6)=="/live/"){
            this.global_.request.getLiveInfo(this,to.params.id).then(() => {
                this.getLiveInfo=this.$root.getLiveInfo;
            });
            this.bitrate_monitor=setInterval(()=>this.global_.request.getLiveInfo(this,to.params.id).then(() => {
                this.getLiveInfo=this.$root.getLiveInfo;
            }),20000);
            this.login=false;
            this.ws_state="连接中";
            this.initWebSocket(to.params.id);
            this.msg_list=[];
            this.id = to.params.id;
            this.room=this.$root.roomList[this.id];
            this.loadRoom(this.id);
            this.$set(this.$root.bread,1,{
                text: this.room.title,
                disabled: true,
                href: '#/live/'+this.$route.params.id,
            });
        }
        next();
    },
    destroyed(){
        if(this.$root.DPlayer) this.$root.DPlayer.destroy();
        if(this.ws){
            this.closeWs=true;
            this.ws.close();
            this.ws=null;
        }
        if(this.reconnect_timer) clearInterval(this.reconnect_timer);
        if(this.heartbeat_timer) clearInterval(this.heartbeat_timer);
        if(this.bitrate_monitor) clearInterval(this.bitrate_monitor);
    }
}
</script>

<style>
@media screen and (max-width:960px){
    .on-phone-chat-box{
        min-height: 400px;
        margin-bottom: 48px!important;
    }
    .on-phone-card-padding-bottom{
        padding-bottom: 16px!important;
    }
    .on-phone-chat-box-flex-reverse{
        flex-direction:column-reverse;
    }
    .on-phone-chat-box-textarea{
        padding-top: 6px;
    }
}
@media screen and (min-width:960px){
    .on-desktop-card-padding-bottom{
        padding-bottom: 50px!important;
    }
    .on-desktop-chat-box-flex{
        flex-direction:column;
    }
}
</style>
