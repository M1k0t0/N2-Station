<script>
import axios from 'axios';
axios.defaults.withCredentials = true;

const BackendAddress="http://live.4g.cx/backend";  // http unsafe
const debugBackendAddress="http://live.4g.cx:8443"
const SFMode=false;
const ERR_MSG={
    'getToken': {
        '-1': '账号或密码错误'
    },
    'openRoom': {
        '-1': '房间不存在',
        '-2': '已到达直播房间数量上限 (1)'
    },
    'closeRoom': {
        '-1': '房间不存在'
    },
    'global': {
        '-10': '答应人家，不要乱动人家的js，好吗？',
        '-11': 'ん? 发生了未知错误！'
    }
}
const get_err_msg=function(action, err_code){
    err_code+=''
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
    openRoom: function(that,id){
        axios
        .post(that.$root.backend+'/api/user/openRoom',{id})
        .then(response => {
            that.$root.openRoom = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
    },
    closeRoom: function(that,id){
        axios
        .post(that.$root.backend+'/api/user/closeRoom',{id})
        .then(response => {
            that.$root.closeRoom = response.data; // full data returned
        })
        .catch(error => {
            console.log(error);
        })
    }
}
export default{
    BackendAddress,
    debugBackendAddress,
    SFMode,
    ERR_MSG,
    get_err_msg,
    request,
    getCookie,
    setCookie,
    delCookie
}
</script>