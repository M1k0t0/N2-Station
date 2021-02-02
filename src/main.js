import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import router from './router'
import global_ from './components/Global';

Vue.config.productionTip = false

new Vue({
  vuetify,
  router,
  render: h => h(App),
  data:() => ({
    backend: global_.SFMode ? global_.BackendAddress : global_.debugBackendAddress,
    sfmode: global_.SFMode,
    flvPlayer: null,
    roomList: {},
    bread: [
      {
          text: 'Homepage',
          disabled: false,
          href: '#/welcome',
      }
    ]
  })
}).$mount('#app')
