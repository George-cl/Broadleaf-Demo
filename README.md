# Broadleaf Demo

**This is a work in progress and there may be large changes made to the current code**

Utilises VueJS and Argon design system to mock up a front-end application which can send messages (signed by the Signer) to the chain.

## Setup
Users will need to install the [CasperLabs Signer](https://chrome.google.com/webstore/detail/casperlabs-signer/djhndpllfiibmcdbnmaaahkhchcoijce) from the Chrome web store before they can use the app fully.
A vault and account must then be created in the Signer extension:  

![Signer Extension](https://user-images.githubusercontent.com/69711689/107745579-6f57eb00-6d0c-11eb-936a-94a3df180f08.png)


Once you have the Signer installed and setup you can then spin up the app.  
Use `yarn` to setup and run a local server for the demo:
```
   $ yarn install
   $ yarn serve
```

## Usage
![Broadleaf-Demo App](https://user-images.githubusercontent.com/69711689/107745389-26079b80-6d0c-11eb-957c-79cbfd3afc92.png)  

Users should **Connect to the Signer** first (although they will be notified if the forget to).
Users can then enter a message and select an emoji that they would like to assign to the message.  
They can then click on **Sign it** - this will send a request for signature to the Signer extension.  

![Signer Notification](https://user-images.githubusercontent.com/69711689/107745769-b219c300-6d0c-11eb-9d99-35a9aff5e771.png)  

Open the extension to approve the request.
At this stage your message has been approved by your account's key but it will need to be signed again by the system key.  
Click on **Send it** to send the message for approval by the system key. (this is still in development).