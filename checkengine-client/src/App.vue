<script setup>
import { useUserStore } from './stores/user';

const store = useUserStore();

</script>

<template>
  <h1>CheckEngine</h1>
  <nav>
    <router-link to="/">Home</router-link> - 
    <router-link to="/login">Login</router-link> - 
    <router-link to="/signup">Sign Up</router-link>
  </nav>
  <div>
    <router-view></router-view>
  </div>
</template>

<style scoped>
</style>
