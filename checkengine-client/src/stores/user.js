import { defineStore } from "pinia";

export const useUserStore = defineStore('user', {
    state: () => ({
        token: 'dfasfasdfasdf',
    }),
    getters: {
        getToken: (state) => state.token,
    },
    actions: {
        setToken(newToken) {
            this.token = newToken;
        },
    },
});