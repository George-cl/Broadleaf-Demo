/*!

=========================================================
* Vue Argon Design System - v1.1.0
=========================================================

* Product Page: https://www.creative-tim.com/product/argon-design-system
* Copyright 2019 Creative Tim (https://www.creative-tim.com)
* Licensed under MIT (https://github.com/creativetimofficial/argon-design-system/blob/master/LICENSE.md)

* Coded by www.creative-tim.com

=========================================================

* The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

*/
import Vue from "vue";
import Landing from "./views/Landing";
import { store } from "./store";
import Argon from "./plugins/argon-kit";


Vue.config.productionTip = false;
Vue.use(Argon);

new Vue({
  store,
  render: h => h(Landing)
}).$mount("#app");
