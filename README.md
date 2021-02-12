# Broadleaf Demo

**This is a work in progress and there may be large changes made to the current code**

Utilises VueJS and Argon design system to mock up a front-end application which can send signed messages (signed by the Signer) to the chain.

## Setup
Users will need to install the [CasperLabs Signer](https://chrome.google.com/webstore/detail/casperlabs-signer/djhndpllfiibmcdbnmaaahkhchcoijce) from the Chrome web store before they can use the app fully.
An vault and account must then be created in the Signer extension:
[SIGNER IMAGE]

Use `yarn` to set it up and run a local server:
```
   $ yarn install
   $ yarn serve
```

## Usage
[DEMO APP IMAGE]  
Users should Connect to the Signer first (although they will be notified if the forget to).
Users can then enter a message and select an emoji that they would like to assign to the message.
They can then click on 'Sign it' - this will send a request for signature to the Signer extension.  
[SIGNER NOTIFICATION]  
Open the extension to approve the request.
At this stage your message has been approved by your account's key but it will need to be signed again by the system key.
Click on 'Send it' to send the message for approval by the system key. (this is still in development).