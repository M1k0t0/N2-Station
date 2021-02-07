import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import router from './router'
import global_ from './components/Global';

Vue.config.productionTip = false
Vue.prototype.global_ = global_

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
    closeRoom: {}
  })
}).$mount('#app')
