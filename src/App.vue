<script setup>
import NaviComponent from './components/NaviComponent.vue';
import { NMessageProvider } from "naive-ui"
import compassMeta from './utils/compassMeta.js'

function init() {
  if (!localStorage.config_network) {
      localStorage.config_network = JSON.stringify({
      compassPriceURL : "https://raw.githubusercontent.com/The-Forbidden-Trove/tft-data-prices/master/lsc/bulk-compasses.json"
    })
  }

  localStorage.compassMeta = JSON.stringify(compassMeta)

}

init()

</script>

<template>
  <NMessageProvider>
    <NaviComponent />
  </NMessageProvider>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
</style>
