<template>
    <base-progress
        id="message-progress" 
        type="info"
        height=12 
        :value=progress
        label="Sending message...">
    </base-progress>
</template>

<script>
import BaseProgress from '../../components/BaseProgress.vue'
export default {
  components: { BaseProgress },
    name: "ProgressBar",
    computed: {
        progress_amount () {
            return this.$store.getters.message_progress
        }
    }
}
</script>

<style>

</style>