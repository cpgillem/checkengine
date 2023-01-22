<script setup>
import { onMounted } from 'vue';
import { useUserStore } from './stores/user';
import router from './router';

const store = useUserStore();

function goHome() {
  if (store.isLoggedIn) {
    router.push('/');
  } else {
    router.push('/login');
  }
}

onMounted(() => {
  goHome();
});
</script>

<template>
  <h1>CheckEngine</h1>
  <nav>
    <!-- <router-link to="/">Home</router-link> -  -->
    <a href="#" @click.prevent="goHome">Home</a> -
    <router-link to="/login">Login</router-link> - 
    <router-link to="/signup">Sign Up</router-link>
  </nav>
  <div>
    <router-view></router-view>
  </div>
</template>

<style scoped>
</style>
