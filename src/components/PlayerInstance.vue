<template>
    <v-container fluid fill-height>
        <v-row
            align="center"
            justify="center"
            class="fill-height"
        >
            <v-col sm="12" md="11" class="fill-height">
                <v-card tile class="pt-3 pl-3 pb-1 pr-3" height="90%">
                    <v-row
                        align="center"
                        justify="center"
                    >
                        <v-col sm="12" md="8" id="videoFrame" class="col-me">
                            <video
                            width="100%"
                            height="auto"
                            id="videoElement" controls autoplay>
                                Your browser is too old which doesn't support HTML5 video.
                            </video>
                        </v-col>
                        <v-spacer />
                        <v-col sm="12" md="4" class="col-me">
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
</template>

<script>

import flvjs from 'flv.js';

export default {
    data: () => ({
        player: {}
    }),
    methods: {
        setSource(id){
            this.player.source = 'http://live.4g.cx/live?port=1935&app=rtmp&stream='+id;
            if(!this.$root.flvPlayer) this.$nextTick(() => this.pullVideo());
            else{
                this.$root.flvPlayer.load();
                this.$root.flvPlayer.play();
            }
        },
        loadRoom(id){
            this.setSource(id);
            this.setChatboxHeight('chatBox');
        },
        pullVideo(){
            if (this.player.source && flvjs.isSupported()) {
                var videoElement = document.getElementById('videoElement');
                this.$root.flvPlayer = flvjs.createPlayer({
                    type: 'flv',
                    url: this.player.source,
                    hasAudio: true,
                    hasVideo: true,
                    isLive: true
                });
                this.$root.flvPlayer.attachMediaElement(videoElement);
                this.$root.flvPlayer.load();
                this.$root.flvPlayer.play();
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
    mounted() {
        this.loadRoom(this.$route.params.id);
    }
}
</script>

