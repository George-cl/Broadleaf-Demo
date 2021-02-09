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
                        <div class="row">
                            <div class="col-lg-6">
                                <h1 class="display-3  text-white">Broadleaf messaging app
                                </h1>
                                <p class="lead  text-white">A simple proof of concept for what Broadleaf's product might
                                    look like built on the Casper blockchain.
                                </p>
                                <form style="margin-top:5%">
                                    <textarea class="form-control form-control-alternative" rows="3" placeholder="Enter message here..."></textarea>
                                </form>
                                <div style="margin-top:8%;margin-bottom:100%">
                                    <div class="btn-wrapper">
                                        <base-button tag="a"
                                                    href="https://demos.creative-tim.com/argon-design-system/docs/components/alerts.html"
                                                    class="mb-3 mb-sm-0"
                                                    type="info"
                                                    icon="fa fa-code">
                                            Sign it
                                        </base-button>
                                        <base-button tag="a"
                                                    href="https://www.creative-tim.com/product/argon-design-system"
                                                    class="mb-3 mb-sm-0"
                                                    type="white"
                                                    icon="ni ni-cloud-download-95">
                                            Send it
                                        </base-button>
                                    </div>
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
export default {
  name: "home",
  components: {}
};
</script>
