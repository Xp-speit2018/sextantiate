<script setup>
import { defineProps, ref, defineEmits, computed, toRefs } from 'vue';
import { NCard, NInput, NSpace, NSwitch, NInputGroup, useMessage, NButton, NButtonGroup} from 'naive-ui';
import debounce from '@/utils/debounce';
import { register, unregister, isRegistered } from '@tauri-apps/api/globalShortcut';
import { invoke } from '@tauri-apps/api/tauri';

window.$message = useMessage()
const message = useMessage()

const props = defineProps(['id', 'data'])
const emit = defineEmits(['save', 'close', 'new'])

// -----------------------------
// input boxes
// -----------------------------
// const data = ref({
//     toggle: '',
//     groups: [
//         { action: '', interval: '' },
//     ]
// })

const { data } = toRefs(props)

// if (props.data) data.value = toRef(props.data).value

const status = computed(()=>{
    let res = {}
    res.toggle = validate(data.value.toggle, toggleInputRegex)
    res.groups = []
    for (let i=0; i<data.value.groups.length; i++) {
        res.groups[i] = {
            action: validate(data.value.groups[i].action, actionInputRegex),
            interval: validate(data.value.groups[i].interval, intervalInputRegex)
        }
    }
    return res
})

const toggleInputRegex = /^((cmd|ctrl|shift|alt|option)\+){0,2}(f[1-9]|f1[0-2])$/
const actionInputRegex = /^((cmd|ctrl|shift|alt|option)\+){0,2}(left|right|[a-z0-9])$/
const intervalInputRegex = /^(?!0)\d+(?:\.\d+)?$/

const validate = (s, regex) => {
    if (!s) return 'warning'
    if (regex.test(s)) return 'success'
    return 'error'
}
const passedRegex = computed(() => {
    if (status.value.toggle != 'success') return false
    for (let item of status.value.groups) {
        if (item.action!='success' || item.interval!='success') return false
    }
    return true
})

const tryEmitSave =  debounce( () => {
    if (passedRegex.value) {
        emit("save", props.id, data.value)
    }
}, 2000)

// -----------------------------
// Button
// -----------------------------
const registered = ref(false)
const activated = ref(false)

const railStyle = ({checked, focused}) => {
    const style = {}
    if (checked) {
        style.background = activated.value ? '#30D030FF' : '#3068D0FF'
        if (focused) {
            style.boxShadow = activated.value ? "0 0 0 2px #30D03040" : '0 0 0 2px #3068D040'
        }
    } 
    return style
}

const press = async (keychain) => {
    const keys = keychain.split('+')
    // message.info(`press keychain ${keychain}`)
    let last = null
    if (keys[keys.length-1]=='left' || keys[keys.length-1]=='right') {
        last = async () => await invoke('mouse_click', {button:keys[keys.length-1]})
    } else {
        last = async () => await invoke('key_click', {key:keys[keys.length-1]})
    }
    if (keys.length==1) {
        await last()
    } else if (keys.length==2) {
        await invoke('key_down', {key:keys[0]})
        await last()
        await invoke('key_up', {key:keys[0]})
    } else if (keys.length==3) {
        await invoke('key_down', {key:keys[0]})
        await invoke('key_down', {key:keys[1]})
        await last()
        await invoke('key_up', {key:keys[1]})
        await invoke('key_up', {key:keys[0]})
    }
}

const handleChange = async () => {
    if (registered.value) {
        await unregister(data.value.toggle)
        registered.value = !registered.value
        message.success(`unregistered hotkey: ${data.value.toggle}`)
    } else {
        if (await isRegistered(data.value.toggle)) {
            message.error('hotkey is registered')
        } else {
            let worker = null
            await register(data.value.toggle, ()=>{
                if (activated.value) {
                    worker.map(item => clearInterval(item))
                } else {
                    for (let item of data.value.groups) press(item.action)
                    worker = data.value.groups.map(item => setInterval(()=>press(item.action), item.interval))
                }
                activated.value = !activated.value
            })
            registered.value = !registered.value
            message.success(`registered hotkey: ${data.value.toggle}`)
        }
    }
}

const plus = () => data.value.groups.push({ action: '', interval: '' })
const minus = () => data.value.groups.pop()

// -----------------------------
// Box
// -----------------------------

const handleClose = () => {
    emit('close', props.id)
}

// -----------------------------
// padding
// -----------------------------

const paddingStyle = ref({
    position: "absolute",
    left: "0",
    top: "0",
    bottom: "0",
    width: "4px",
    background_color: "rgba(128, 128, 128, 0.5)",
    cursor: "pointer"
})
const paddingStyleString = computed(()=>{
    let res = ""
    for (let key in paddingStyle.value) {
        let val = paddingStyle.value[key]
        if (key=="background_color") key = "background-color"
        res += key + ':' + val + ';'
    }
    return res
})

let enterInterval = null
let leaveInterval = null
const interval = 15
const step = 1
const onMouseEnter = async () => {
    if (leaveInterval!=null) clearInterval(leaveInterval)
    enterInterval = setInterval(()=>{
        const px = Number(paddingStyle.value.width.match(/\d+/g))
        if (px+step<=8) {
            paddingStyle.value.width = String(px+step)+'px'
        } else clearInterval(enterInterval)
    }, interval)
}
const onMouseLeave = async () => {
    if (enterInterval!=null) clearInterval(enterInterval)
    leaveInterval = setInterval(()=>{
        const px = Number(paddingStyle.value.width.match(/\d+/g))
        if (px-step>=4) {
            paddingStyle.value.width = String(px-step)+'px'
        } 
        else clearInterval(leaveInterval)
    }, interval)
}

</script>

<template>

<NCard embedded hoverable size="small" header-style="text-align: left"
    :closable="!registered"
    @close="handleClose"
    style="padding-left: 12px; position: relative; width: 90%; margin: auto;"
>   
    <div
      :style="paddingStyleString"
      @click="emit('new', props.id)"
      @mouseenter="onMouseEnter"
      @mouseleave="onMouseLeave"
    />
    <template #header>
        <NSwitch 
            :value="registered"
            :rail-style="railStyle"
            :disabled="!passedRegex || activated"
            @update:value="handleChange"
        />
        <NButtonGroup>
            <NButton circle @click="minus" :disabled="data.groups.length<=1 || registered">-</NButton>
            <NButton circle @click="plus" :disabled="registered">+</NButton>
        </NButtonGroup> 
    </template>
    <NSpace v-for="(item, index) in data.groups" :key="index">
        <NInputGroup >
            <NInput clearable placeholder="toggle"
                :status="status.toggle" 
                :style="{width:'30%'}"
                v-model:value="data.toggle"
                @update:value="tryEmitSave"
                :disabled="registered || index>0"
            />
            <NInput clearable placeholder="action"
                :status="status.groups[index].action"
                :style="{width:'40%'}"
                v-model:value="item.action"
                @update:value="tryEmitSave"
                :disabled="registered"
            />
            <NInput clearable placeholder="interval"
                :status="status.groups[index].interval"
                :style="{width:'30%'}"  
                v-model:value="item.interval" 
                @update:value="tryEmitSave"
                :disabled="registered"
            >
                <template #suffix>ms</template>
            </NInput>
        </NInputGroup>
    </NSpace>
</NCard>


</template>