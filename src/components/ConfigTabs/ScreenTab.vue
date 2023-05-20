<script setup>
import { ref } from 'vue'
import { NCollapse, NCollapseItem, NInput, useMessage } from 'naive-ui'
import debounce from '@/utils/debounce';

window.$message = useMessage()
const message = useMessage()
const config = ref({})
if (localStorage.config_string) {
    config.value = JSON.parse(localStorage.config_string)
}

async function save() {
  localStorage.config_string = JSON.stringify(config.value)
  message.success('saved')
}


</script>

<template>
<NCollapse accordion :oninput="debounce(save, 2000)">
    <NCollapseItem title="compassPriceURL">
        <NInput 
        clearable 
        placeholder="provided by tft"
        v-model:value="config.compassPriceURL"
        default-value="https://raw.githubusercontent.com/The-Forbidden-Trove/tft-data-prices/master/lsc/bulk-compasses.json"
        />
    </NCollapseItem>
    <NCollapseItem title="compassPriceUpdateProxy">
        <NInput 
        clearable 
        placeholder="Not implemented"
        v-model:value="config.IGN"
        />
    </NCollapseItem>
    <NCollapseItem title="stashUpdateProxy">
        <NInput 
        clearable 
        placeholder="Not implemented" 
        v-model:value="config.realm"
        />
    </NCollapseItem>
</NCollapse>

</template>