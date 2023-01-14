import {createStore} from 'vuex';

export default createStore({
    state: {
        token: "",
    },
    mutations: {
        // Sets the JWT.
        setToken(state, payload) {
            state.token = payload;
        },
    },
    actions: {

    },
    getters: {
        // Retrieves the JWT for a request.
        getToken(state) {
            return state.token;
        }
    },
    modules: {

    },
});