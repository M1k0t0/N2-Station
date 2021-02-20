<template>
    <v-app id="sandbox">

        <v-system-bar app color="black" class="ml-2">
            <span><b>N2Station</b></span>
            <v-spacer></v-spacer>
        </v-system-bar>

        <v-navigation-drawer
        style="padding-top:12px;"
        color="grey darken-4"
        v-model="primaryDrawer.model"
        app
        width="300"
        floating
        >
            <!-- <v-navigation-drawer
                    v-model="primaryDrawer.model"
                    :clipped="primaryDrawer.clipped"
                    :floating="primaryDrawer.floating"
                    :mini-variant="primaryDrawer.mini"
                    mini-variant-width="70"
                    :expand-on-hover="false"
                    absolute
                    color="grey darken-4"
            > -->
            <v-row justify="center" class="fill-height">
                <v-col cols="3" color="grey darken-4" class="pa-0">
                <v-avatar
                class="d-block text-center mt-0 mx-6"
                style="border-radius: 14.97px !important;"
                color="grey darken-1"
                size="47"
                rounded
                @click="clearSource(); updateRoomList(); routeTo('/welcome');"
                ><v-icon>fa-hashtag</v-icon></v-avatar>

                <v-divider class="ml-6 mr-3 my-5"></v-divider>

                <transition-group 
                name="flip-list" 
                tag="div"
                >
                    <v-badge 
                    bordered
                    overlap
                    v-for="item in sortedRoomList"
                    :key="item.title"
                    class="text-center ml-6 mr-3"
                    :color="item.status=='open'?'red':'grey'" 
                    :icon="item.status=='open'?'mdi-broadcast':'mdi-broadcast-off'"
                    >
                        <v-avatar
                        link
                        class="d-block text-center mx-auto mb-6"
                        color="grey lighten-1"
                        size="40"
                        >
                            <v-tooltip right>
                                <template v-slot:activator="{ on, attrs }">
                                    <v-btn 
                                    v-bind="attrs"
                                    v-on="on"
                                    icon 
                                    @click="routeTo('/live/',$root.sfmode?item.stream_id:item.id)"
                                    style="margin-top:1px;">
                                        <!-- <img
                                            src=""
                                            style="width:40px!important; height:40px!important; margin-top:2px;"
                                            class="ml-5"
                                        > -->
                                        
                                        <!-- <v-icon v-if="item.status=='open'" class="mx-auto my-auto">mdi-broadcast</!-->
                                        <!-- <v-icon v-if="item.status=='close'" class="mx-auto my-auto">mdi-broadcast-off</v-icon> -->
                                    </v-btn>
                                </template>
                                <span>{{item.title}}</span>
                            </v-tooltip>
                        </v-avatar>
                    </v-badge>
                </transition-group>
            </v-col>
            <!-- </v-navigation-drawer> -->
            <v-col cols="9" class="pa-0">
            <v-sheet
            color="grey darken-4"
            height="70"
            width="100%"
            align="center"
            justify="center"
            tile
            style="padding-top:7px;margin-left:-12px;"
            >
            <v-card flat style="background-color: transparent !important;" class="mt-n1">
                <v-responsive max-width="200" min-width="80%">
                    <v-text-field
                    dense
                    flat
                    hide-details
                    rounded
                    solo-inverted
                    label="搜索"
                    style="margin-top:0px; margin-bottom:13px;"
                    @blur="on_search=true"
                    width="100%"
                    ></v-text-field>
                </v-responsive>

                <!-- <v-snackbar
                    v-model="error_snackbar"
                    top
                    absolute
                    width="80%"
                    >
                        Not a vaild RoomID.
                </v-snackbar>

                <v-text-field 
                v-model="player.id"
                append-outer-icon='mdi-bus'
                @click:append-outer="goToRoom()"
                label="快速进入房间"
                class="mt-0 pt-0 pl-4 pr-3"
                color="white"
                >

                </v-text-field> -->
            </v-card>
            </v-sheet>

            <router-view name="listTagItems"></router-view>
            
            </v-col>
            </v-row>
        </v-navigation-drawer>

        <v-app-bar
            color="grey darken-4"
            :clipped-left="primaryDrawer.clipped"
            app
        >
        <v-row justify="space-between">
            <v-col xs="3" md="3" sm="1" class="my-auto">
                <v-btn
                    @click.stop="primaryDrawer.model = !primaryDrawer.model;  primaryDrawer.model2=!primaryDrawer.model2;"
                    v-if="!primaryDrawer.model"
                    icon
                >
                    <v-icon>fa-chevron-right</v-icon>
                </v-btn>
                <v-btn
                    @click.stop="primaryDrawer.model = !primaryDrawer.model; primaryDrawer.model2=!primaryDrawer.model2;"
                    v-if="primaryDrawer.model"
                    icon
                >
                    <v-icon>fa-chevron-left</v-icon>
                </v-btn>
            </v-col>

            <v-col xs="4" md="4" sm="10" class="col-me-manual-50">
                <div class="text-center">
                    <v-breadcrumbs :items="$root.bread" color="white" style="color: white!important;"></v-breadcrumbs>
                </div>
            </v-col>
            <v-spacer></v-spacer>
            <v-col xs="1" md="1" sm="1" class="my-auto text-end">
                <v-btn icon @click="routeTo('/login')">
                    <v-icon>mdi-login-variant</v-icon>
                </v-btn>
            </v-col>
        </v-row>
        </v-app-bar>

        <v-content>
            <router-view></router-view>
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

// import axios from 'axios';
// import global_ from './components/Global';

export default {
    name: "App",
    components:{ },
    data: () => ({
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
            id: null,
            error: null
        },
        error_snackbar: false,
        on_search:false,
        timerID: null
    }),
    methods:{
        clearSource(){
            if(this.$root.flvPlayer){
                this.$root.flvPlayer.destroy();
                this.$root.flvPlayer=null;
            }
        },
        updateRoomList(){
            this.global_.request.getRoomList(this);
        },
        goToRoom(){
            if(this.$root.roomList[this.player.id]==undefined)
                this.error_snackbar=true;
            else
                this.routeTo('/live/',this.player.id)
        },
        routeTo(base, data=''){
            this.$router.push({
                path: base+data,
            })
        }
    },
    created () {
        if(this.$route.path=='/')
            this.routeTo('/welcome');
        this.global_.request.getRoomList(this);
        this.timerID=setInterval(() => {
            this.global_.request.getRoomList(this);
        }, 600000);
    },
    computed: {
        sortedRoomList() {
            return Object.values(this.$root.roomList).slice().sort((e1,e2) => {
                return e1.status=="open"?-1:e2.status=="open"?1:0;
            });
        }
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
.col-me-manual-50 {
    flex-basis: 50% !important;
}
.v-application a {
    color: inherit !important;
}
.v-badge__badge .v-icon {
    font-size: 12px !important;
}
.flip-list-move {
    transition: transform 1s;
}
.no-outline {
    outline: none;
}
::-webkit-scrollbar {
  display: none; /* Chrome Safari */
}
.element {
    scrollbar-width: none; /* Firefox */ 
    overflow: -moz-scrollbars-none;
    -ms-overflow-style: none; /* IE 10+ */
}
</style>