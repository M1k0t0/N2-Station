import Vue from 'vue'
import VueRouter from 'vue-router'
import Welcome from '@/components/Welcome'
import PlayerInstance from '@/components/PlayerInstance'
import LoginForm from '@/components/LoginForm'
import TagSort from '@/components/TagSort'

Vue.use(VueRouter)

const routes = [
  {
    path: '/welcome',
    name: 'Welcome',
    component: Welcome
  },
  {
    path: '/live/:id',
    name: 'PlayerInstance',
    component: PlayerInstance
  },
  {
    path: '/login',
    name: 'LoginForm',
    component: LoginForm
  },
  {
    path: '/tag/:tag',
    name: 'TagSort',
    component: TagSort
  }
]
//component: () => import(/* webpackChunkName: "about" */ '../views/About.vue')
const router = new VueRouter({
  routes
})

export default router
