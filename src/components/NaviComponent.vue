<script setup>
import { h, ref } from 'vue';
import { NIcon, NMenu, NSpace, NLayout, NLayoutSider } from "naive-ui"
import { RouterLink } from 'vue-router';
import {
    HomeOutline as HomeIcon,
    SettingsOutline as ConfigIcon,
    CompassOutline as PricingIcon,
    FlashOutline as HotkeyIcon,
    BarChartOutline as StatsIcon
} from "@vicons/ionicons5";

const activeKey =  ref(null);
const collapsed = ref(true);

function renderIcon(icon) {
    return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions = [
    {
        label: () => h(RouterLink, { to: '/' }, { default: ()=>"Home" }),
        key: "home",
        icon: renderIcon(HomeIcon)
    },
    {
        label: () => h(RouterLink, { to: '/config' }, { default: ()=>"Config" }),
        key: "config",
        icon: renderIcon(ConfigIcon)
    },
    {
        label: () => h(RouterLink, { to: '/pricing' }, { default: ()=>"Pricing" }),
        key: "pricing",
        icon: renderIcon(PricingIcon)
    },
    {
        label: () => h(RouterLink, { to: '/hotkeys'}, { default: ()=>"Hotkeys" }),
        key: "hotkeys",
        icon:renderIcon(HotkeyIcon)
    },
    {
        label: () => h(RouterLink, { to: '/stats'}, { default: ()=>"Stats" }),
        key: "stats",
        icon:renderIcon(StatsIcon)
    }
]

</script>

<template>
<NSpace vertical>
    <!-- <NSwitch v-model:value="collapsed" /> -->
    <NLayout has-sider position="absolute">
        <NLayoutSider
            collapse-mode="width"
            :collapsed-width="64"
            :width="160"
            :collapsed="collapsed"
            :native-scrollbar="false"
            show-trigger
            @collapse="collapsed=true"
            @expand="collapsed=false"
        >
            <NMenu 
                v-model:value="activeKey"
                :collapsed="collapsed"
                :collapsed-width="64"
                :collapsed-icon-size="22"
                :options="menuOptions"
            />
        </NLayoutSider>
        <NLayout embedded :native-scrollbar="false">
            <RouterView v-slot="{ Component }">
                <!-- <Transition> -->
                    <KeepAlive>
                        <component :is="Component" />
                    </KeepAlive>
                <!-- </Transition> -->
            </RouterView>
        </NLayout>
    </NLayout>
</NSpace>

</template>

<style>

</style>