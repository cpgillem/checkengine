import {createRouter, createWebHistory} from 'vue-router';
// import Home from '../components/pages/Home';
// import LoginForm from '../components/pages/LoginForm';
// import SignupForm from '../components/pages/SignupForm';

const routes = [
    // {
    //     path: '/',
    //     name: 'MainApp',
    //     component: Home,
    // },
    // {
    //     path: '/login',
    //     name: 'LoginForm',
    //     component: LoginForm,
    // },
    // {
    //     path: '/signup',
    //     name: 'SignupForm',
    //     component: SignupForm,
    // },
];

const router = createRouter({
    history: createWebHistory(process.env.BASE_URL),
    routes
});

export default router;