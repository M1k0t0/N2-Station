<template>
    <v-app id="sandbox">

        <v-navigation-drawer
                v-model="primaryDrawer.model"
                :clipped="primaryDrawer.clipped"
                :floating="primaryDrawer.floating"
                :mini-variant="primaryDrawer.mini"
                mini-variant-width="60"
                :expand-on-hover="true"
                app
                overflow
        >

            <v-list-item
                    link
                    @click="clearSource()"
            >
                <v-list-item-icon>
                    <v-icon>fa-hashtag</v-icon>
                </v-list-item-icon>

                <v-list-item-content>
                    <v-list-item-title>主页面</v-list-item-title>
                </v-list-item-content>
            </v-list-item>

            <v-divider></v-divider>

            <v-list-item style="min-height:38px">
                <v-list-item-subtitle>
                    频道
                </v-list-item-subtitle>
            </v-list-item>

            <v-list
                    dense
                    nav
                    class="pt-0"
            >
                <v-list-item
                        v-for="item in roomList"
                        :key="item.title"
                        link
                        @click="loadRoom(item.id)"
                >
                    <v-list-item-icon v-if="item.status=='open'">
                        <v-icon class="mt-2">mdi-broadcast</v-icon>
                    </v-list-item-icon>

                    <v-list-item-icon v-if="item.status=='close'">
                        <v-icon class="mt-2">mdi-broadcast-off</v-icon>
                    </v-list-item-icon>

                    <v-list-item-content>
                    <v-list-item-title>{{ item.title }}</v-list-item-title>
                    <v-list-item-subtitle>{{ item.desc }}</v-list-item-subtitle>
                    </v-list-item-content>

                </v-list-item>
            </v-list>
        </v-navigation-drawer>

        <v-app-bar
                :clipped-left="primaryDrawer.clipped"
                app
        >

        <v-btn
                @click.stop="primaryDrawer.model = !primaryDrawer.model"
                v-if="!primaryDrawer.model"
                icon
        >
            <v-icon>fa-chevron-right</v-icon>
        </v-btn>
        <v-btn
                @click.stop="primaryDrawer.model = !primaryDrawer.model"
                v-if="primaryDrawer.model"
                icon
        >
            <v-icon>fa-chevron-left</v-icon>
        </v-btn>
        <v-toolbar-title>
            N2 Station
        </v-toolbar-title>

        <v-spacer></v-spacer>

        <v-btn icon>
            <v-icon>mdi-login-variant</v-icon>
        </v-btn>

        </v-app-bar>

        <v-content>
            <v-container fluid>
                <v-row
                        align="center"
                        justify="center"
                >
                <v-col sm="12" md="10" v-if="!player.source">
                    <v-card shaped tile>
                        <div class="pl-4 pb-2 pt-2">
                            <h1># N2 Station</h1>
                            <br />
                            <p>一个实现 N2 Frontend API 的多直播源直播站</p>
                            <p>Github项目: <a href="https://github.com/M1k0t0/N2-Station">https://github.com/M1k0t0/N2-Station</a></p>
                            <p>拉流播放器基于 <a href="https://github.com/bilibili/flv.js">flv.js</a></p>
                            <p>项目仍在编写中，觉得顶的话就给个Star吧！</p>
                        </div>
                    </v-card>
                </v-col>
                
                <v-col sm="12" md="11" v-if="player.source">
                    <v-card tile class="pt-3 pl-3 pb-1 pr-3">
                        <v-row
                            align="center"
                            justify="center"
                        >
                            <v-col sm="12" md="8" v-if="player.source" id="videoFrame" class="col-me">
                                <video
                                width="100%"
                                height="auto"
                                id="videoElement" controls autoplay>
                                    Your browser is too old which doesn't support HTML5 video.
                                </video>
                            </v-col>
                            <v-spacer />
                            <v-col sm="12" md="4" v-if="player.source" class="col-me">
                                <v-card tile id="chatBox" class="pt-1 pb-1" height="80%">
                                    <v-card outline id="chatContent" height="80%" class="ml-2 mr-2 mt-0 mb-2">
                                    </v-card>
                                    <v-row class="py-5">
                                    <v-col cols="9" class="pt-0 pr-0">
                                        <v-text-field color="white" label="Commit" class="ml-2 mr-2 pt-0 mt-0"></v-text-field>
                                    </v-col>
                                    <v-spacer/>
                                    <v-col cols="3" class="pt-0 pr-0 pl-4">
                                        <v-btn icon color="white"><v-icon color="white">mdi-arrow-up-thick</v-icon></v-btn>
                                    </v-col>
                                    </v-row>
                                </v-card>
                            </v-col>
                        </v-row>
                    </v-card>
                </v-col>
                </v-row>
            </v-container>
        </v-content>

        <v-footer
                :inset="footer.inset"
                app
        >
            <span class="px-4">N2 Station &copy; {{ new Date().getFullYear() }}</span>
        </v-footer>
    </v-app>
</template>

<script>

import axios from 'axios';
import flvjs from 'flv.js';

export default {
    name: "App",
    components:{ },
    data: () => ({
        backend: "http://live.4g.cx:8443",  // http unsafe
        primaryDrawer: {
            model: true,
            type: 'permanent',
            clipped: true,
            floating: true,
            mini: true,
        },
        footer: {
            inset: false,
        },
        player:{
            options: { },
            source: ""
        },
        roomList: {}
    }),
    methods:{
        clearSource(){
            this.player.source = '';
            if(this.flvPlayer){
                this.flvPlayer.destroy();
                this.flvPlayer=null;
            }
        },
        setSource(id){
            this.player.source = 'http://live.4g.cx/live?port=1935&app=rtmp&stream='+id;
            if(!this.flvPlayer) this.$nextTick(() => this.pullVideo());
            else{
                this.flvPlayer.load();
                this.flvPlayer.play();
            }
        },
        loadRoom(id){
            this.setSource(id);
            this.setChatboxHeight('chatBox');
        },
        pullVideo(){
            if (this.player.source && flvjs.isSupported()) {
                var videoElement = document.getElementById('videoElement');
                this.flvPlayer = flvjs.createPlayer({
                    type: 'flv',
                    url: this.player.source,
                    hasAudio: true,
                    hasVideo: true,
                    isLive: true
                });
                this.flvPlayer.attachMediaElement(videoElement);
                this.flvPlayer.load();
                this.flvPlayer.play();
            }
        },
        setChatboxHeight(id){
            this.$nextTick(() => {
                if(document.documentElement.clientWidth>=960){
                    document.getElementById(id).style.height=document.getElementById('videoFrame').clientHeight-24+'px';
                }else
                    document.getElementById(id).style.height=document.documentElement.clientHeight-document.getElementById('videoFrame').clientHeight-24+'px';
            })
        }
    },
    mounted () {
        axios
        .get(this.backend+'/api/info/room')
        .then(response => {
            this.roomList = response.data.data
        })
        .catch(error => {
            console.log(error)
            this.errored = true
        })
        .finally(() => this.loading = false);
    }
}
</script>
<style lang="less">
    * {
        margin: 0;
        padding: 0;
    }
    .remove-text{
        text-align: center;
        padding: 20px;
        font-size: 24px;
    }
    .player-btns{
        width: 100%;
        display: flex;
        justify-content: center;
        flex-wrap: wrap;
        span {
            margin: 0 auto;
            display: inline-block;
            padding: 5px 10px;
            width: 150px;
            height: 40px;
            line-height: 40px;
            border: 1px solid #eee;
            background: #e1e1e1;
            border-radius: 10px;
            text-align: center;
            margin: 5px;
            cursor: pointer;
        }
    }
    .source-box{
        padding: 5px 10px;
        margin-bottom: 20px;
        .source-label{
            margin-right: 20px;
            font-size: 16px;
            display: block;
        }
        #source{
            margin-top: 10px;
        }
        .source-input{
            margin-top: 10px;
            padding: 5px 10px;
            width: 80%;
            border: 1px solid #ccc;
        }
    }
</style>
<style>
/* 用于覆盖错误的 CSS 样式 */
.col-me {
    flex-basis: auto !important;
}
</style>