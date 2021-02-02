<template>
    <v-app id="sandbox">

        <v-system-bar app color="black" class="ml-2">
            <span><b>N2Station</b></span>
            <v-spacer></v-spacer>
        </v-system-bar>

        <v-navigation-drawer
            color="grey darken-3"
            v-model="primaryDrawer.model"
            app
            width="300"
            floating
            >
            <v-navigation-drawer
                    v-model="primaryDrawer.model"
                    :clipped="primaryDrawer.clipped"
                    :floating="primaryDrawer.floating"
                    :mini-variant="primaryDrawer.mini"
                    mini-variant-width="70"
                    :expand-on-hover="false"
                    absolute
                    color="grey darken-4"
            >
                <v-avatar
                class="d-block text-center mx-auto mt-4"
                color="grey darken-1"
                size="47"
                rounded
                @click="clearSource(); routeTo('/welcome')"
                ><v-icon>fa-hashtag</v-icon></v-avatar>

                <v-divider class="mx-3 my-5"></v-divider>
                
                <div 
                v-for="item in $root.roomList"
                :key="item.title"
                class="text-center">
                    <v-badge 
                    bordered
                    avatar
                    overlap
                    :color="item.status=='open'?'red':'grey'" 
                    :icon="item.status=='open'?'mdi-broadcast':'mdi-broadcast-off'"
                    >
                        <v-avatar
                        link
                        class="d-block text-center mx-auto mb-6"
                        color="grey lighten-1"
                        size="40"
                        >
                        
                            <v-btn 
                            icon 
                            @click="$root.sfmode?routeTo('/live/',item.stream_id):routeTo('/live/',item.id)"
                            style="margin-top:1px;">
                                <img
                                    src=""
                                    style="width:40px!important; height:40px!important; margin-top:2px;"
                                    class="ml-5"
                                >
                                
                                <!-- <v-icon v-if="item.status=='open'" class="mx-auto my-auto">mdi-broadcast</!-->
                                <!-- <v-icon v-if="item.status=='close'" class="mx-auto my-auto">mdi-broadcast-off</v-icon> --> -->
                            </v-btn>
                        </v-avatar>
                    </v-badge>
                </div>
            </v-navigation-drawer>

            <v-sheet
            color="grey darken-4"
            height="120"
            width="100%"
            align="center"
            justify="center"
            style="padding-left:73px;"
            tile
            >
            <v-card flat>
                <v-subheader>快速进入</v-subheader>
                <v-snackbar
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
                label="RoomID"
                class="mt-0 pt-0 pl-4 pr-3"
                color="white"
                >

                </v-text-field>
            </v-card>
            </v-sheet>

            <v-list
                style="padding-left:75px;"
                shaped
            >
                <v-list-item style="min-height:38px" v-if="tagList.open.length">
                    <v-list-item-title class="text-center">
                        活跃
                    </v-list-item-title>
                </v-list-item>

                <v-list-item style="min-height:38px" v-if="!tagList.open.length">
                    <v-list-item-title class="text-center">
                        活跃
                    </v-list-item-title>
                </v-list-item>
                <v-list-item style="min-height:38px" v-if="!tagList.open.length">
                    <v-list-item-content>
                        <v-list-item-subtitle>暂无此类型的Tag</v-list-item-subtitle>
                    </v-list-item-content>
                </v-list-item>

                <v-list-item
                v-for="tag in tagList.open"
                :key="tag"
                link
                @click="routeTo('/tag/',tag)"
                >
                <v-list-item-content>
                    <v-list-item-subtitle># {{ tag }}</v-list-item-subtitle>
                </v-list-item-content>
                </v-list-item>

                <v-list-item style="min-height:38px" v-if="tagList.close.length">
                    <v-list-item-title class="text-center">
                        不活跃
                    </v-list-item-title>
                </v-list-item>

                <v-list-item style="min-height:38px" v-if="!tagList.close.length">
                    <v-list-item-title class="text-center">
                        不活跃
                    </v-list-item-title>
                    
                </v-list-item>
                <v-list-item style="min-height:38px" v-if="!tagList.close.length">
                    <v-list-item-content>
                        <v-list-item-subtitle>暂无此类型的Tag</v-list-item-subtitle>
                    </v-list-item-content>
                </v-list-item>

                <v-list-item
                v-for="tag in tagList.close"
                :key="tag"
                link
                @click="routeTo('/tag/',tag)"
                >
                <v-list-item-content>
                    <v-list-item-subtitle># {{ tag }}</v-list-item-subtitle>
                </v-list-item-content>
                </v-list-item>
            </v-list>
        </v-navigation-drawer>

        <v-app-bar
            color="grey darken-4"
            :clipped-left="primaryDrawer.clipped"
            app
        >
        <v-row justify="space-between">
            <v-col xs="3" md="3" class="my-auto">
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
            </v-col>

            <v-col xs="4" md="4">
                <div class="text-center">
                    <v-breadcrumbs :items="$root.bread" color="white" style="color: white!important;"></v-breadcrumbs>
                </div>
            </v-col>
            <v-spacer></v-spacer>
            <v-col xs="1" md="1" class="my-auto text-end">
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

import axios from 'axios';
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
        tagList:{ 'open': [], 'close': [] },
        error_snackbar: false
    }),
    methods:{
        clearSource(){
            if(this.$root.flvPlayer){
                this.$root.flvPlayer.destroy();
                this.$root.flvPlayer=null;
            }
        },
        getRoomList(){
            axios
            .get(this.$root.backend+'/api/info/room')
            .then(response => {
                this.$root.roomList = this.$root.sfmode ? response.data.rooms : response.data.data;
            })
            .catch(error => {
                console.log(error);
                this.errored = true;
            })
            .finally(() => this.loading = false);
        },
        getTagList(){
            axios
            .get(this.$root.backend+'/api/info/tag')
            .then(response => {
                this.tagList = response.data.data;
            })
            .catch(error => {
                console.log(error);
                this.errored = true;
            })
            .finally(() => { this.loading = false; });
        },
        routeTo(base, data=''){
            this.$router.push({
                path: base+data,
            })
        },
        goToRoom(){
            if(this.$root.roomList[this.player.id]==undefined)
                this.error_snackbar=true;
            else
                this.routeTo('/live/',this.player.id)
        }
    },
    mounted () {
        this.getRoomList();
        this.getTagList();
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
.v-application a {
    color: inherit !important;
}
</style>