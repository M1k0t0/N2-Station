<template>
<v-container fill-height class="pa-0 align-center justify-center">
    <v-snackbar
    absolute
    top
    :timeout=1145141919
    v-model="sb"
    >
        {{ sb_msg }}
        <v-btn
            color="pink"
            text
            @click="sb=false;"
        >
            Close
        </v-btn>
    </v-snackbar>
    <v-stepper
    v-model="e6"
    vertical
    class="ma-12 pt-4 pb-4"
    style="background-color:#36393f; width:90%;"
    non-linear
    >
    <v-stepper-step
        :rules="[() => e6=='1'||validID]"
        :complete="e6 > 1"
        step="1"
        color="#7289da"
        editable
        class="py-5 mb-1"
    >
        房间ID
        <small>也是推流ID，不可更改。请不要用奇怪的字符，中文也不可以哦</small>
    </v-stepper-step>

    <v-stepper-content step="1">
        <v-text-field
        :rules="[hasValidID()]"
        autofocus
        v-model="id"
        class="mr-8"
        solo
        label="e.g. ckLive"
        :counter="16"
        clearable
        prepend-inner-icon="mdi-barcode"
        ></v-text-field>
        <v-btn
        color="#7289da"
        @click="e6 = 2"
        >
        下一步
        </v-btn>
    </v-stepper-content>

    <v-stepper-step
        :rules="[() => e6<=2 || (validTitle && validDesc)]"
        :complete="e6 > 2"
        step="2"
        color="#7289da"
        editable
        class="py-5 my-1"
    >
        标题、描述
    </v-stepper-step>

    <v-stepper-content step="2">
        <v-text-field
        :rules="[hasValidTitle()]"
        v-model="title"
        class="mr-8"
        solo
        label="e.g. 全聚德"
        :counter="16"
        clearable
        prepend-inner-icon="mdi-buddhism"
        ></v-text-field>
        <v-textarea
        :rules="[hasValidDesc()]"
        v-model="desc"
        class="mr-8"
        solo
        label="e.g. 这是一个直播烤鸭子的直播间"
        :counter="64"
        clearable
        prepend-inner-icon="mdi-content-paste"
        ></v-textarea>
        
        <v-btn
        color="#7289da"
        @click="e6 = 3"
        >
        下一步
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>

    <v-stepper-step
        :complete="e6 > 3"
        step="3"
        color="#7289da"
        editable
        class="py-5 my-1"
    >
        直播标签
        <small>请查阅主页的tag列表，尽量减少同义tag数量哦</small>
    </v-stepper-step>

    <v-stepper-content step="3">
        <v-chip
        class="ma-2"
        v-for="t of tagList"
        :key="t"
        :close="t!='默认'"
        @click:close="delTag(t)"
        >
        {{ t }}
        </v-chip>
        <v-text-field
        :rules="[v => (!v || v.length <= 8) || 'Tag长度必须<=8']"
        autofocus
        v-model="tag"
        class="mt-5 mr-8"
        solo
        label="e.g. 鸭子"
        :counter="8"
        clearable
        prepend-inner-icon="mdi-tag"
        append-outer-icon="mdi-database-import"
        @click:append-outer="addTag();"
        ></v-text-field>
        <v-btn
        color="#7289da"
        @click="e6 = 4"
        >
        下一步
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>

    <v-stepper-step 
    step="4"
    color="#7289da"
    editable
    class="py-5 my-1"
    >
        缩略图
        <small>可能会重新设计，暂定64x64</small>
    </v-stepper-step>
    <v-stepper-content step="4">
        <v-card
        color="grey lighten-1"
        class="mb-12"
        height="200px"
        ></v-card>
        <v-btn
        color="#7289da"
        @click="createRoom()"
        :loading="loading"
        :disable="loading"
        >
        创建
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>
    </v-stepper>
</v-container>
</template>
<script>
import axios from 'axios';

export default {
    data: () => ({
        e6: 1,
        id: null,
        title: null,
        desc: null,
        tag: null,
        tagList:["默认"],
        validID: false,
        validTitle: false,
        validDesc: true,
        createRoomRet: {},
        loading: false,
        sb_msg: '',
        sb: false
    }),
    methods: {
        addTag(){
            if(!this.tag || this.tag.length>=8) return;
            if(this.tagList[0]=="默认") this.$set(this.tagList,0,this.tag);
            if(this.tagList.indexOf(this.tag)==-1)
                this.tagList.push(this.tag);
            this.tag='';
        },
        delTag(tag){
            var index=this.tagList.indexOf(tag);
            if(index!=-1)
                this.tagList.splice(index,1);
            if(this.tagList.length==0) this.$set(this.tagList,0,'默认');
        },
        hasValidID(){
            let v=this.id;
            this.validID=false;
            if(v==null) return true;
            if(v=='') return '请输入房间ID';
            if(v.length<4 || v.length>16) return '房间ID必须为4-16个字符';
            if(/[^0-9a-zA-Z_]/.test(v)) return '房间ID不能包含特殊字符';
            this.validID=true;
            return true;
        },
        hasValidTitle(){
            let v=this.title;
            this.validTitle=false;
            if(v==null) return true;
            if(v=='') return '请输入标题';
            if(v.length>16) return '房间标题长度必须<=16';
            this.validTitle=true;
            return true;
        },
        hasValidDesc(){
            let v=this.desc;
            if(v==null) return true;
            this.validDesc=false;
            if(v.length>64) return '房间描述长度必须<=64';
            this.validDesc=true;
            return true;
        },
        createRoom(){
            this.loading=true;
            axios
            .post(this.$root.backend+'/api/user/createRoom',{
                'id':this.id,
                'title':this.title,
                'desc':this.desc,
                'image':'default',
                'tag':this.tagList
            })
            .then(response => {
                this.createRoomRet=response.data;
            })
            .catch(error => {
                console.log(error);
            })
            .finally(() => {
                this.checkStatus();
            })
        },
        checkStatus(){
            if(this.createRoomRet.status!=0){
                this.loading=false;
                this.sb_msg=this.global_.get_err_msg(this.createRoomRet.action,this.createRoomRet.status);
                this.sb=true;
            }else{
                this.sb_msg='创建成功';
                this.sb=true;
                setTimeout(() => this.routeTo('/panel/rooms'), 1000);
            }
        },
        routeTo(base, data=''){
            this.$router.push({
                path: base+data,
            })
        }
    }
}
</script>
