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
                                    <p class="text-white">Emoji: {{ emoji.unicode }}</p>
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
                                                    @click="sendMessage(message)"
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
import { DeployUtil, encodeBase16, Signer, CasperClient } from "casper-client-sdk";
import { toBytesString } from "casper-client-sdk/dist/lib/byterepr";
import { Picker } from "emoji-picker-element";
import { mapState } from "vuex";


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
      }
  },
  computed: {
      message() {
          return this.messageText;
      },
      emoji() {
          return this.selectedEmoji;
      },
      ...mapState({
          progress: state => state.msg_progress,
          label: state => state.msg_label,
          approvals: state => state.msg_approvals
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
      
      createDeploy(message) {
          DeployUtil.deploy //TODO
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

          let serializedMsg = toBytesString(message);
          let base16Msg = encodeBase16(serializedMsg);
          let base64PublicKey = await Signer.getSelectedPublicKeyBase64();
          
          if (this.progress > 29 && this.progress < 40) {
              this.incrementProgress();
              this.setProgressLabel("Signing message");
          }
          // returns base64 signature from the Signer
          let sigResponse = await Signer.sign(base16Msg, base64PublicKey);

          if (sigResponse && this.progress > 39 && this.progress < 50) {
              this.incrementProgress();
              this.setProgressLabel("Message signed");
          }

          this.addApproval(sigResponse);
          return sigResponse;
      },

      async systemSignMessage(message) {
          let client = new CasperClient(
              'http://localhost:40101',
              'http://localhost:3000'
          );

          let serializedMsg = toBytesString(message);
          let base16Msg = encodeBase16(serializedMsg);
          let base64PublicKey = await Signer.getSelectedPublicKeyBase64();

          if (this.progress > 49 && this.progress < 60) {
              this.incrementProgress();
              this.setProgressLabel("System key signing");
          }

          client.signDeploy()
          let sigResponse = await Signer.sign(base16Msg, base64PublicKey);
          if (sigResponse && this.progress > 59 && this.progress < 70) {
              this.incrementProgress();
              this.setProgressLabel("Signed by system key");
          }

          this.addApproval(sigResponse);
          return sigResponse;
      },

      async constructDeploy() {
          if (this.approvals.length < 2) {
              console.error("Insufficient approvals to create deploy (<2): " + this.approvals.length);
              return
          }
          let json = {
              base16message: encodeBase16(toBytesString(this.message)),
              approvals: this.approvals,
              emoji: this.selectedEmoji
          }

          return DeployUtil.deployFromJson(json);
      },

      async putDeploy(deploy) {
          let client = new CasperClient(
              'http://localhost:40101',
              'http://localhost:3000'
          )
          return await client.putDeploy(deploy);
      },

      async sendMessage(message) {
          await this.systemSignMessage(message);
          deploy = await this.constructDeploy();
          deployHash = await this.putDeploy(deploy);
          await this.checkDeploy(deployHash);
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
          console.log(event);
      },

      setEmoji(emoji) {
          let chosenEmoji = {
              annotation: emoji.annotation,
              unicode: emoji.unicode
          };
          this.selectedEmoji = chosenEmoji;
      },

      async checkDeploy (deployHash) {
          let client = new CasperClient(
              'http://localhost:40101',
              'http://localhost:3000'
          );
          response = await client.getDeployByHash(deployHash);
          console.log(response);
          return response;
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