# Broadleaf Demo

Utilises VueJS and Argon design system to mock up a front-end application which can send signed (by the Sgner) messaeges to the chain.

Message structure:

    {
        message: "This is the message but it will be encoded in base16"
    }

Deploy structure:

    {
        approvals: [
            base64encodedsignature,     // USER key
            base64encodedsignature      // SYSTEM key
        ],
        message: "This is the message but it will be encoded in base16",
        emoji: "smiling_face" // Unicode identifier for emoji
    }