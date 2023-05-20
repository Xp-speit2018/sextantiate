<script setup>
import { ref } from 'vue'
import { NCollapse, NCollapseItem, NInput, useMessage } from 'naive-ui'
import debounce from '@/utils/debounce';

window.$message = useMessage()
const message = useMessage()

const config = ref({})
if (localStorage.config_account) config.value = JSON.parse(localStorage.config_account)

async function save() {
  localStorage.config_account = JSON.stringify(config.value)
  message.success('saved')
}

</script>

<template>

<NCollapse accordion :oninput="debounce(save, 2000)">
    <NCollapseItem title="account name">
        <NInput 
        clearable 
        placeholder="Your account name, not character name"
        v-model:value="config.accountName"
        />
    </NCollapseItem>
    <NCollapseItem title="In-Game-Name(IGN)">
        <NInput 
        clearable 
        placeholder="Your character name for others to contact you on discord"
        v-model:value="config.IGN"
        />
    </NCollapseItem>
    <NCollapseItem title="realm">
        <NInput 
        clearable 
        placeholder="platform you play on" 
        v-model:value="config.realm"
        />
    </NCollapseItem>
    <NCollapseItem title="league">
        <NInput
        clearable
        placeholder="league you play on, generally sc but not necessarily"
        v-model:value="config.league"    
        />
    </NCollapseItem>
    <NCollapseItem title="POESESSID">
        <NInput
        clearable
        type="password"
        placeholder="Careful!"
        show-password-on="click"
        v-model:value="config.POESESSID"
        />
    </NCollapseItem>
</NCollapse>

</template>