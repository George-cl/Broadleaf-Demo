import { sign } from "casper-client-sdk/dist/lib/Signer";
import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export const store = new Vuex.Store({
  state: {
    msg_progress: 0,
    msg_label: "",
    msg_approvals: [],
  },
  getters: {
      message_progress (state) {
          return state.msg_progress
      },
      message_approvals (state) {
          return state.msg_approvals
      }
  },
  mutations: {
      inc_progress (state) {
          state.msg_progress += 10;
      },

      res_progress (state) {
          state.msg_progress = 0;
      },

      set_lbl (state, lblText) {
          state.msg_label = lblText;
      },

      add_approval (state, sig) {
          state.msg_approvals.push(sig);
      },

      clr_approvals (state) {
          state.msg_approvals = [];
      },

  },
  actions: {
      increment_progress({commit}) {
          commit('inc_progress');
      },

      reset_progress({commit}) {
          commit('res_progress');
      },

      set_label({commit}, labelText) {
          commit('set_lbl', labelText);
      },

      add_approval({commit}, signature) {
          commit('add_approval', signature);
      },

      clear_approvals({commit}) {
          commit('clr_approvals');
      }
  }
});