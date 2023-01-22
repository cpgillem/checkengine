import { defineStore } from "pinia";

export const useUserStore = defineStore('user', {
    state: () => ({
        token: '',
    }),
    getters: {
        getToken: (state) => state.token,
        isLoggedIn: (state) => {
            // TODO: Check timeout?
            return state.token.length > 0;
        }
    },
    actions: {
        setToken(newToken) {
            this.token = newToken;
        },
    },
});