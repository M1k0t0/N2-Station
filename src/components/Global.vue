<script>
import axios from 'axios';
axios.defaults.withCredentials = true;

const BackendAddress="https://live.4g.cx/backend";  // http unsafe
const debugBackendAddress="https://n2station.live:8443";
const pullAddressHTTPPort=443;
const pullAddressRTMPPort=1935;
const pullAddress="https://live.4g.cx:"+pullAddressHTTPPort;
const pushAddress="rtmp://publish.n2station.live:"+pullAddressRTMPPort;
const SFMode=false;
const ERR_MSG={
    'getToken': {
        '-1': '账号或密码错误'
    },
    'register': {
        '-1': '注册失败，邮箱已存在',
        '-2': '注册失败，用户名已存在',
        '-3': '注册失败，密码强度不足 (你不应该看到这个哦)',
        '-4': '注册失败，邀请码错误'
    },
    'openRoom': {
        '-1': '房间不存在',
        '-2': '已到达直播房间数量上限 (1)'
    },
    'closeRoom': {
        '-1': '房间不存在'
    },
    'createRoom': {
        '-1': '相同房间已存在',
        '-2': '已达房间数量上限'
    },
    'deleteRoom': {
        '-1': '房间不存在'
    },
    'global': {
        '-10': 'Token已失效或参数错误',
        '-11': 'ん? 发生了未知错误！'
    }
}
const get_err_msg=function(action, err_code){
    err_code+='';
    if(ERR_MSG[action]!=undefined && ERR_MSG[action][err_code]!=undefined) return ERR_MSG[action][err_code];
    if(ERR_MSG['global'][err_code]!=undefined) return ERR_MSG['global'][err_code];
}
const getCookie=function(cname){
    var name = cname + "=";
    var ca = document.cookie.split(';');
    for(var i=0; i<ca.length; i++) 
    {
    var c = ca[i].trim();
    if (c.indexOf(name)==0) return c.substring(name.length,c.length);
    }
    return "";
}
const setCookie=function(cname,cvalue,exdays=90){
    var d = new Date();
    d.setTime(d.getTime()+(exdays*24*60*60*1000));
    var expires = "expires="+d.toGMTString();
    document.cookie = cname + "=" + cvalue + "; " + expires;
}
const delCookie=function(cname){
    var d=new Date();
    d.setTime(0);
    var expires = "expires="+d.toGMTString();
    document.cookie = cname + "=" + "; " + expires;
}
const request={
    getRoomList: function(that){
        axios
        .get(that.$root.backend+'/api/info/room',{withCredentials:true})
        .then(response => {
            that.$set(that.$root,'roomList',that.$root.sfmode ? response.data.rooms : response.data.data);
        })
        .catch(error => {
            console.log(error);
        });
    },
    asyncGetRoomList: async function(that){
        await axios
        .get(that.$root.backend+'/api/info/room',{withCredentials:true})
        .then(response => {
            that.$set(that.$root,'roomList',that.$root.sfmode ? response.data.rooms : response.data.data);
        })
        .catch(error => {
            console.log(error);
        });
    },
    getTagList: function(that){
        axios
        .get(that.$root.backend+'/api/info/tag')
        .then(response => {
            that.$root.tagList = response.data.data;
        })
        .catch(error => {
            console.log(error);
        })
    },
    getUserRoomList: async function(that){
        axios
        .get(that.$root.backend+'/api/user/rooms')
        .then(response => {
            that.$root.userRoomList = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
        .finally(() => {
            return true;
        })
    },
    openRoom: async function(that,id){
        await axios
        .post(that.$root.backend+'/api/user/openRoom',{id})
        .then(response => {
            that.$root.openRoom = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
    },
    closeRoom: async function(that,id){
        await axios
        .post(that.$root.backend+'/api/user/closeRoom',{id})
        .then(response => {
            that.$root.closeRoom = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
    },
    deleteRoom: async function(that,id){
        await axios
        .post(that.$root.backend+'/api/user/deleteRoom',{id})
        .then(response => {
            that.$root.deleteRoom = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
    }
}
export default{
    BackendAddress,
    debugBackendAddress,
    pullAddress,
    pushAddress,
    pullAddressHTTPPort,
    pullAddressRTMPPort,
    SFMode,
    ERR_MSG,
    get_err_msg,
    request,
    getCookie,
    setCookie,
    delCookie
}
</script>