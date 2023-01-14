import {createRouter, createWebHistory} from 'vue-router';
import Home from '../components/pages/Home.vue';
import LoginForm from '../components/pages/LoginForm.vue';
import SignupForm from '../components/pages/SignupForm.vue';

const routes = [
    {
        path: '/',
        name: 'Home',
        component: Home,
    },
    {
        path: '/login',
        name: 'LoginForm',
        component: LoginForm,
    },
    {
        path: '/signup',
        name: 'SignupForm',
        component: SignupForm,
    },
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
});

export default router;