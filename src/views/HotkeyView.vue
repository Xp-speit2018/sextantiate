<script setup>
import { unregisterAll } from '@tauri-apps/api/globalShortcut';
import { NSpace, NButton, NEmpty, useMessage } from 'naive-ui';

import LoopComponent from '@/components/HotkeyComponent/LoopComponent.vue';
import sextantComponent from '@/components/HotkeyComponent/sextantComponent.vue';
import { ref, computed, onBeforeMount, onBeforeUnmount } from 'vue';
window.$message = useMessage()
const message = useMessage()

onBeforeMount(() => {
    unregisterAll()
}),
onBeforeUnmount(() => {
    unregisterAll()
})

const loops = ref([])
const nodata = computed(()=>loops.value.length==0)
const sortedloops = computed(()=>[...loops.value].sort((a,b)=>a.id-b.id))
const createNewLoop = () => {
    loops.value.push({
        id: 0,
        saved: false,
        data: {
            toggle: '',
            groups: [
                { action: '', interval: '' },
            ]
        }
    })
}

if (localStorage.loops) loops.value = JSON.parse(localStorage.loops)

const save = () => {
    localStorage.loops = JSON.stringify(
        loops.value.filter(item=>item.saved)
    )
    message.success('saved')
}

const handleNew = (id) => {
    let sortedId = sortedloops.value.map(item=>item.id)
    const sortedIdIndex = sortedId.indexOf(id)
    let newId = 0
    if (sortedIdIndex == sortedId.length-1) {
        newId = (1+id)/2
    } else {
        newId = (id+sortedId[sortedIdIndex+1])/2
    }
    loops.value.push({
        id: newId,
        saved: false,
        data: {
            toggle: '',
            groups: [
                { action: '', interval: '' },
            ]
        }
    })
}
const handleClose = (id) => {
    const ids = loops.value.map(item=>item.id)
    const index = ids.indexOf(id)
    let saved = loops.value[index].data.saved
    if (saved) console.log(`deleting hotkey group ${loops.value[index]}`)

    loops.value.splice(index, 1)
    save()
}
const handleSave = (id, data) => {
    const ids = loops.value.map(item=>item.id)
    const index = ids.indexOf(id)
    loops.value[index].data=data
    loops.value[index].saved=true
    save()
}

</script>

<template>
    <sextantComponent/>
    <NEmpty v-if="nodata" description="No hotkey yet." style="margin-top: 20%;">
        <template #extra>
            <NButton @click="createNewLoop">New Loop Group</NButton>
        </template>
    </NEmpty>

    <NSpace vertical size="large" v-for="(item, index) in sortedloops" :key="item.id">
        <span v-if="index" />
        <LoopComponent 
        :id="item.id"
        :data="item.data" 
        @close="handleClose"
        @new="handleNew"
        @save="handleSave"
        />
    </NSpace>

</template>