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
import { DeployUtil, Signer, CasperClient, RuntimeArgs } from "casper-client-sdk";
import { Picker } from "emoji-picker-element";
import { mapState } from "vuex";
import { DeployParams } from 'casper-client-sdk/dist/lib/DeployUtil';
import { Ed25519, SignatureAlgorithm } from 'casper-client-sdk/dist/lib/Keys';
import { readFileSync } from 'fs';

let client = new CasperClient(
    'http://localhost:40101',
    'http://localhost:3000'
)

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
          casperNodePath: "/home/ethilios/CasperLabs/casper-node/",
          nodeAddress: this.casperNodePath + "utils/nctl/assets/net-1/nodes/node-1/",
          sessionCodePath: "../../storage-contract/target/wasm32-unknown-unknown/release/contract.wasm"
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
      
      makeStorageContractDeploy(accountPublicKey, args = null) {
          // Allows for access to the Vue instance in other scopes
          var vm = this;
          try {
              
              let deployParams = new DeployParams(accountPublicKey, "casper-net-1");
              let sessionCode = new Uint8Array(readFileSync(vm.sessionCodePath, null).buffer);
              let runtimeArgs = RuntimeArgs.fromMap(args)
              let sessionModule = DeployUtil.ExecutableDeployItem.newModuleBytes(sessionCode, runtimeArgs);
              let payment = DeployUtil.standardPayment(100000000000);
              
              return DeployUtil.makeDeploy(
                  deployParams,
                  sessionModule,
                  payment
              );

          } catch {
              throw new Error("Failed to create deploy!");
          }
      },

      addSignatureToDeploy(deploy, signingKey) {
          // return deploy with given signature appended to approvals
          return DeployUtil.signDeploy(deploy, signingKey);
      },

      async deployStorageContract() {
        //   let publicKey = client.loadPublicKeyFromFile(this.nodeAddress + "/keys/public_key.pem", SignatureAlgorithm.Ed25519);
          try {
              var publicKeyContent = readFileSync(this.nodeAddress + "/keys/public_key.pem").toString();
          } catch (error) {
              console.error(error);
          };
          console.log(publicKeyContent);
          let publicKey = Ed25519.readBase64WithPEM(publicKeyContent);
          console.log(publicKey);
          let keyPair = Ed25519.parseKeyFiles(this.nodeAddress + "/keys/public_key.pem", this.nodeAddress + "/keys/secret_key.pem");
          let unsignedDeploy = this.makeStorageContractDeploy(publicKey);
          await this.printDeploy(unsignedDeploy.hash);
          let signedDeploy = this.addSignatureToDeploy(unsignedDeploy, keyPair);
          await this.printDeploy(signedDeploy.hash);

          return await client.putDeploy(signedDeploy);
      },

      async printDeploy(deployHash) {
          console.log(await client.getDeployByHash(deployHash));
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