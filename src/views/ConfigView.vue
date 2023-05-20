<script setup>
import { NTabs, NTabPane } from "naive-ui";
import AccountTab from "@/components/ConfigTabs/AccountTab.vue";
import NetworkTab from "@/components/ConfigTabs/NetworkTab.vue";
import ScreenTab from "@/components/ConfigTabs/ScreenTab.vue";

</script>

<template>
    <NTabs animated>
        <NTabPane name="account" tab="Account">
            <AccountTab/>
        </NTabPane>
        <NTabPane name="network" tab="Network">
            <NetworkTab/>
        </NTabPane>
        <NTabPane name="screen" tab="Screen">
            <ScreenTab/>
        </NTabPane>
    </NTabs>
    
    
</template>

<style> 

</style>