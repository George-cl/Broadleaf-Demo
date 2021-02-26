<template>
    <div>
        <div class="position-relative">
            <!-- shape Hero -->
            <section class="section-shaped my-0" style="height: 100vh;">
                <div class="shape shape-style-1 shape-default">
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
                <div class="container shape-container d-flex">
                    <div class="col px-0">
                        <div style="margin-left:80%;margin-bottom:10%;">
                            <button type="button" class="btn btn-info" v-on:click="connectSigner()">
                                Connect to Signer
                            </button>
                            <button type="button" class="btn btn-primary" v-on:click="deployStorageContract()" style="margin-top:1rem">
                                Deploy Contract
                            </button>                            
                        </div>
                        <div class="row">
                            <div class="col-md-6">
                                <h1 class="display-3  text-white">Broadleaf messaging app
                                </h1>
                                <p class="lead text-white">A simple proof of concept for what Broadleaf's product might
                                    look like built on the Casper blockchain.
                                </p>
                                <form style="margin-top:5%">
                                    <p class="text-white">Emoji: {{ emoji.unicode }}</p>
                                    <textarea v-model="messageText" id="messageText" class="form-control form-control-alternative" rows="3" placeholder="Enter message here..."></textarea>
                                </form>
                                <div style="margin-top:8%;margin-bottom:100%">
                                    <div class="btn-wrapper">
                                        <base-button tag="a"
                                                    v-on:click="createMessage(message)"
                                                    class="mb-3 mb-sm-0"
                                                    type="info"
                                                    icon="ni ni-key-25">
                                            Sign it
                                        </base-button>
                                        <base-button tag="a"
                                                    @click="sendMessage()"
                                                    class="mb-3 mb-sm-0"
                                                    type="white"
                                                    icon="ni ni-send">
                                            Send it
                                        </base-button>
                                        <base-button tag="a"
                                                    @click="toggleEmojis()"
                                                    class="mb-3 mb-sm-0"
                                                    type="white"
                                                    icon="ni ni-satisfied">
                                        </base-button>
                                    </div>
                                    <base-progress
                                        v-if="progress > 0"
                                        id="message-progress" 
                                        type="info"
                                        height=12
                                        :value=this.progress
                                        :label=this.label>
                                    </base-progress>
                                </div>
                            </div>
                            <div class="col-md-6">
                                <div style="margin-top:20%;" id="emoji-wrapper">
                                    <!-- Emoji-picker will be inserted here on mount -->
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    </div>
</template>

<script>
import { Picker } from "emoji-picker-element";
import { mapState } from "vuex";

import { CLValue, Signer } from "casper-client-sdk";
import { buildMessengerDeploy, sendMessengerDeploy } from "../deploy-utils";

// Set faucet key
const faucetPublicKey = '013037f71429b16b5c28af7f7175b5298500e30a8b654521d64eed4d39af118691';

export default {
  name: "home",
  mounted() {
      (() => {
          const picker = new Picker();
          document.getElementById("emoji-wrapper").appendChild(picker);
          picker.addEventListener('emoji-click', event => {
              this.handleEmojiClick(event.detail);
          });
      })();
  },
  data() {
      return {
          messageText: "",
          selectedEmoji: "",
          messageDeploy: "",
          processedDeployHash: "",
      }
  },
  computed: {
      message() {
          return this.messageText;
      },
      emoji() {
          return this.selectedEmoji;
      },
      deploy() {
          return this.messageDeploy;
      },
      deployHash() {
          return this.processedDeployHash;
      },
      ...mapState({
          progress: state => state.msg_progress,
          label: state => state.msg_label,
          approvals: state => state.msg_approvals
      }),
  },
  methods: {

      // SIGNER METHODS
      async checkConnected() {
          return await Signer.isConnected();
      },

      async connectSigner() {
          if (await this.checkConnected()) {
              alert("Already connected to Signer");
              return;
          }
          Signer.sendConnectionRequest();
      },

      async getUserKey() {
          if (await this.checkConnected()) {
              return await Signer.getSelectedPublicKeyBase64();
          }
          alert("Please connect to the Signer first!");
          return;
      },

      async createMessage() {
          let key = await this.getUserKey();
          let msg = this.message;
          let emj = this.emoji.annotation;

          this.messageDeploy = buildMessengerDeploy(this.faucetPublicKey, {
              sender: CLValue.string(key),
              message: CLValue.string(msg),
              emoji: CLValue.string(emj)
          });
      },

      async sendMessage() {
          this.processedDeployHash = await sendMessengerDeploy(this.messageDeploy, this.faucetPublicKey);
      },

      // HELPERS  

      toggleEmojis() {
          let emojis = document.getElementById("emoji-wrapper");
          let toggle = emojis.style.display == "none"
            ? "block"
            : "none"
          emojis.style.display = toggle;
          if (toggle == "block" && this.progress > 9 && this.progress < 20) {
              this.incrementProgress();
              this.setProgressLabel("Choosing emoji");
          }
      },

      handleEmojiClick(event) {
          this.toggleEmojis();
          if (this.progress > 19 && this.progress < 30) {
              this.incrementProgress();
              this.setProgressLabel("Emoji Selected");
          }
          this.setEmoji(event.emoji)
          console.log(this.emoji.annotation);
      },

      setEmoji(emoji) {
          let chosenEmoji = {
              annotation: emoji.annotation,
              unicode: emoji.unicode
          };
          this.selectedEmoji = chosenEmoji;
      },

      // STORE INTERACTIONS
      incrementProgress() {
          this.$store.dispatch('increment_progress');
      },

      resetProgress() {
          this.$store.dispatch('reset_progress');
      },

      setProgressLabel(text) {
          this.$store.dispatch('set_label', text);
      },

      addApproval(signature) {
          this.$store.dispatch('add_approval', signature);
      },

      clearApprovals() {
          this.$store.dispatch('clear_approvals');
      },

      clearTextBox() {
          document.getElementById("messageText").value = "";
      },
  },
  watch: {
      message: function () {
          if (this.message.length == 0 && this.progress <= 10) {
              this.setProgressLabel("");
              this.resetProgress();
              return;
          }
          if (this.progress >= 10) {
              return
          }
          this.setProgressLabel("Typing message")
          this.incrementProgress();
      }
  },
};
</script>

<style>

    #emoji-wrapper {
        display: none;
    }
    .emoji-picker {
        transition: ease-in;
        width:100%;
        --emoji-size: 1.5rem;
    }
    #message-progress .progress-label span {
        color: #5e72e4;
        background-color: azure;
    }
    #message-progress .progress-percentage span {
        color: azure;
    }
</style>