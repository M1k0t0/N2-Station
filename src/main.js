import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import Gravatar from 'vue-gravatar';
import lineClamp from 'vue-line-clamp'
import router from './router'
import global_ from './components/Global';

Vue.config.productionTip = false;
Vue.prototype.global_ = global_;
Vue.component('v-gravatar', Gravatar);
Vue.use(lineClamp, {
  importCss: true
})

new Vue({
  vuetify,
  router,
  render: h => h(App),
  data:() => ({
    backend: global_.SFMode ? global_.BackendAddress : global_.debugBackendAddress,
    sfmode: global_.SFMode,
    flvPlayer: null,
    roomList: {},
    tagList:{ 'open': [], 'close': [] },
    bread: [
      {
          text: 'Homepage',
          disabled: false,
          href: '#/welcome',
      }
    ],
    userRoomList: {},
    openRoom: {},
    closeRoom: {},
    deleteRoom: {},
    panelMenuIndex: 0
  })
}).$mount('#app')
