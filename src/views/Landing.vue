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
                                    <textarea v-model="messageText" class="form-control form-control-alternative" rows="3" placeholder="Enter message here..."></textarea>
                                </form>
                                <div style="margin-top:8%;margin-bottom:100%">
                                    <div class="btn-wrapper">
                                        <base-button tag="a"
                                                    v-on:click="signMessage(message)"
                                                    class="mb-3 mb-sm-0"
                                                    type="info"
                                                    icon="ni ni-key-25">
                                            Sign it
                                        </base-button>
                                        <base-button tag="a"
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
                                </div>
                            </div>
                            <div class="col-md-6">
                                <div style="margin-top:20%;" id="emoji-container">
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
import { encodeBase16, Signer } from "casper-client-sdk";
import { toBytesString } from "casper-client-sdk/dist/lib/byterepr";
import "emoji-picker-element";

export default {
  name: "home",
  data() {
      return {
          messageText: "",
          selectedEmoji: document.querySelector("emoji-picker")
              .addEventListener('emoji-click', event => console.log(event.detail)),
      }
  },
  computed: {
      message() {
          return this.messageText;
      },
    //   emoji() {
    //       return this.selectedEmoji;
    //   }
  },
  methods: {
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

      constructMessage(messageText, emoji) {

      },

      async signMessage(message) {

          let serializedMsg = this.serializeString(message);
          let base16Msg = encodeBase16(serializedMsg);
          let base64PublicKey = await Signer.getSelectedPublicKeyBase64();

          Signer.sign(base16Msg, base64PublicKey);
      },

      sendMessage() {

      },

      serializeString(string) {
          return toBytesString(string);
      },

      toggleEmojis() {
          let emojis = document.getElementById("emojis");
          let toggle = emojis.style.display == "none"
            ? "block"
            : "none"
          emojis.style.display = toggle;
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
</style>