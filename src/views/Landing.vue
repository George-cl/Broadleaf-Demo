<template>
    <div>
        <div class="position-relative">
            <!-- shape Hero -->
            <section class="section-shaped my-0">
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
                        </div>
                        <div class="row">
                            <div class="col-md-6">
                                <h1 class="display-3  text-white">Broadleaf messaging app
                                </h1>
                                <p class="lead text-white">A simple proof of concept for what Broadleaf's product might
                                    look like built on the Casper blockchain.
                                </p>
                                <form style="margin-top:5%">
                                    <textarea v-model="messageText" id="messageText" class="form-control form-control-alternative" rows="3" placeholder="Enter message here..."></textarea>
                                </form>
                                <div style="margin-top:8%;margin-bottom:100%">
                                    <div class="btn-wrapper">
                                        <base-button tag="a"
                                                    v-on:click="userSignMessage(message)"
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
                                                    v-on:click="toggleEmojis()"
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
                                <div style="margin-top:20%;">
                                    <emoji-picker class="light emoji-picker" id="emojis"></emoji-picker>
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
import { DeployUtil, encodeBase16, Signer } from "casper-client-sdk";
import { toBytesString } from "casper-client-sdk/dist/lib/byterepr";
import "emoji-picker-element";
import { mapGetters, mapState } from "vuex";


export default {
  name: "home",
  data() {
      return {

          systemKeys:{
              publicKey: 1,
              secretKey: 1,
              publicKeyHex: 1 
          },

          messageText: "",
        //   selectedEmoji: document.querySelector("emoji-picker")
        //     .addEventListener('emoji-click', event => console.log(event.detail))
      }
  },
  computed: {
      message() {
          return this.messageText;
      },
      ...mapGetters('progress', {
          messageProgress: 'message_progress'
      }),
      ...mapState({
          progress: state => state.msg_progress,
          label: state => state.msg_label,
      })
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

      async userSignMessage(message) {
        //   this.clearTextBox();
          if (await this.checkConnected() == false) {
              alert("Please connect to the Signer first");
              return
          } else if (this.message == "") {
              alert("Please enter a message first");
              return
          }

          let serializedMsg = this.serializeString(message);
          let base16Msg = encodeBase16(serializedMsg);
          let base64PublicKey = await Signer.getSelectedPublicKeyBase64();
          let deploy = DeployUtil
          
          if (this.progress > 19 && this.progress < 30) {
              this.incrementProgress();
              this.setProgressLabel("Signing message");
          }
          // returns base64 signature from the Signer
          let sigResponse = await Signer.sign(base16Msg, base64PublicKey);

          if (sigResponse && this.progress > 29 && this.progress < 40) {
              this.incrementProgress();
              this.setProgressLabel("Message signed");
          }
      },

      async systemSignMessage(message) {
          
          let serializedMsg = this.serializeString(message);
          let base16Msg = encodeBase16(serializedMsg);
          let base64PublicKey = await Signer.getSelectedPublicKeyBase64();

          Signer.sign(base16Msg, base64PublicKey);
      },

      sendMessage() {
          this.incrementProgress();
      },

      // HELPERS  
      serializeString(string) {
          return toBytesString(string);
      },

      toggleEmojis() {
          let emojis = document.getElementById("emojis");
          let toggle = emojis.style.display == "none"
            ? "block"
            : "none"
          emojis.style.display = toggle;
          if (toggle == "block" && this.progress > 9 && this.progress < 20) {
              this.incrementProgress();
              this.setProgressLabel("Choosing emoji");
          }
      },

      handleEmoji(event) {
          console.log(event.detail);
      },

      incrementProgress() {
          this.$store.dispatch('increment_progress');
      },

      resetProgress() {
          this.$store.dispatch('reset_progress');
      },

      setProgressLabel(text) {
          this.$store.dispatch('set_label', text);
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
  }
};
</script>

<style>

    #emojis {
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