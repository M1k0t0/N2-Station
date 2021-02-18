import Vue from 'vue'
import VueRouter from 'vue-router'
import Welcome from '@/components/Welcome'
import PlayerInstance from '@/components/PlayerInstance'
import LoginForm from '@/components/LoginForm'
import TagSort from '@/components/TagSort'
import TagItems from '@/components/TagItems'
import UserPanel from '@/components/UserPanel'
import PanelMenu from '@/components/PanelMenu'
import PanelRoomList from '@/components/PanelRoomList'
import PanelCreateRoom from '@/components/PanelCreateRoom'
import PanelEditRoom from '@/components/PanelEditRoom'
import PanelChangeInfo from '@/components/PanelChangeInfo'
import PanelChangePassword from '@/components/PanelChangePassword'
import Panel2FA from '@/components/Panel2FA'

Vue.use(VueRouter)

const routes = [
  {
    path: '/welcome',
    name: 'Welcome',
    components: {
      default: Welcome,
      listTagItems: TagItems
    }
  },
  {
    path: '/live/:id',
    name: 'PlayerInstance',
    components: {
      default: PlayerInstance,
      listTagItems: TagItems
    }
  },
  {
    path: '/login',
    name: 'LoginForm',
    components: {
      default: LoginForm,
      listTagItems: TagItems
    }
  },
  {
    path: '/tag/:tag',
    name: 'TagSort',
    components: {
      default: TagSort,
      listTagItems: TagItems
    }
  },
  {
    path: '/listTagItems',
    name: 'listTagItems',
    component: TagItems
  },
  {
    path: '/panel',
    name: 'UserPanel',
    components: {
      default: UserPanel,
      listTagItems: PanelMenu
    },
    children: [
      {
        path: 'rooms',
        component: PanelRoomList
      },
      {
        path: 'createRoom',
        component: PanelCreateRoom
      },
      {
        path: 'editRoom',
        component: PanelEditRoom
      },
      {
        path: 'changeInfo',
        component: PanelChangeInfo
      },
      {
        path: 'changePassword',
        component: PanelChangePassword
      },
      {
        path: '2FA',
        component: Panel2FA
      }
    ]
  }
]
//component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
const router = new VueRouter({
  routes
})

export default router
