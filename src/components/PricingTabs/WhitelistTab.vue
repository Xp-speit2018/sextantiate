<script setup>
import { NDataTable, NCard, NButton, useMessage, NInput } from 'naive-ui';
import { ref, computed } from 'vue';
import debounce from '@/utils/debounce';
import { fetch } from '@tauri-apps/api/http'
import { readText } from '@tauri-apps/api/clipboard'

window.$message = useMessage()
const message = useMessage()

const updating = ref(false);
const data = ref([]);
const checkedRowKeys = ref([]);
const whitelistTitle = ref("Please update compass price");
const key2mod = JSON.parse(localStorage.compassMeta).key2mod;
const mod2weight = JSON.parse(localStorage.compassMeta).mod2weight;

const markedData = computed(()=>
    data.value
    .map(item => {
        if (!(item.name in key2mod)) return null
        const res = {}
        res.name = item.name
        res.chaos = item.chaos
        res.chaosString = item.chaos.toString()
        if (item.lowConfidence) res.chaosString += "*"
        return res
    })
    .filter(item => item!=null)
    .map(item => {
        const res = item
        res.weight = mod2weight[key2mod[item.name]]
        return res
    })
)



const columns = [
    { type: "selection" },
    { title: 'Name', key: 'name' },
    { title: 'Chaos', key: 'chaosString' },
    { title: 'Weight', key: 'weight' }
]

function preprocess(j) {
    const res = {
          timestamp: j['timestamp'],
          data: j['data'].sort((a, b) => b['chaos']-a['chaos'])
    }
    return res
}

async function load() {
    if (!localStorage.price) {
        // message.info('could not find local data, please update first')
        return
    }
    const j = JSON.parse(localStorage.price)
    whitelistTitle.value = "Updated at "+new Date(j['timestamp']).toLocaleString()
    data.value = j['data']
    // message.success('Compass price loaded, timestamp: '+new Date(j['timestamp']).toLocaleString())
    if (!localStorage.whitelist) return
    checkedRowKeys.value = JSON.parse(localStorage.whitelist)
}

async function update_compass_price() {
    updating.value = true
    const compassPriceURL = JSON.parse(localStorage.config_network).compassPriceURL
    try {
        const j = (await fetch(compassPriceURL)).data; 
        if (localStorage.price && JSON.parse(localStorage.price).timestamp == j.timestamp) {
            message.info('Already latest');
            return
        }
        const res = preprocess(j);
        
        whitelistTitle.value = "Updated at "+new Date(res['timestamp']).toLocaleString()
        data.value = res['data']
        localStorage.price = JSON.stringify(res)
        message.success("Compass price updated!")
    } catch (error) {
        message.error(error)
    } finally{
        updating.value = false;
    }
}

async function update_from_clipboard() {
    try {
        const j = JSON.parse(await readText())
        if (localStorage.price && JSON.parse(localStorage.price).timestamp == j.timestamp) {
            message.info('Already latest');
            return
        }
        const res = preprocess(j);
        // console.log(res)
        whitelistTitle.value = "Updated at "+new Date(res['timestamp']).toLocaleString()
        data.value = res['data']
        localStorage.price = JSON.stringify(res)
        message.success("Compass price updated from clipboard!")
    } catch (error) {
        message.error(error)
    }
    
}

const save_whitelist = debounce(()=>{
    localStorage.whitelist = JSON.stringify(checkedRowKeys.value)
    message.success('whitelist saved')
}, 2000)

function handleCheck(keys) {
    checkedRowKeys.value = keys
    save_whitelist()
}

load()

// sextant profit estimate
const sextantPriceInput = ref('')
const sextantPriceInputValue = computed(()=>{
    if (sextantPriceInputStatus.value=="success") {
        return eval(sextantPriceInput.value)
    } else {
        return 0
    }
})
const sextantPriceInputStatus = computed(()=>{
    const regex = /^(\d+\s*[+\-*/]\s*)*\d+$/
    if (regex.test(sextantPriceInput.value)) {
        
        return "success"
    }
    return "warning"
})
const netProfit = computed(()=>{
    const weights = markedData.value.map(item=>item.weight)
    const profits = markedData.value.map(item=>{
        if ( checkedRowKeys.value.indexOf(item.name)!=-1 ) {
            return item.chaos-1-sextantPriceInputValue.value
        } else {
            return -sextantPriceInputValue.value
        }
    })

    const weightedSum = profits.reduce((acc, val, i) => acc + val * weights[i], 0)
    const sumOfWeights = weights.reduce((acc, val) => acc + val, 0);
    const weightedMean = weightedSum / sumOfWeights;

    const variance = profits.reduce((acc, val, i) => acc + weights[i]*(val-weightedMean)**2 ) / sumOfWeights;
    const standardDeviation = Math.sqrt(variance);

    return [weightedMean, standardDeviation]
})

const netProfitMessage = computed(()=>{
    return `Net profit per sextant: ${netProfit.value[0].toFixed(2)} ± ${netProfit.value[1].toFixed(2)} chaos`
})

</script>

<template>
    <NCard :title="whitelistTitle" embedded hoverable>
    <template #header-extra>
        <NButton  :loading="updating" @click="update_compass_price"> Fetch latest update </NButton>
        <NButton @click="update_from_clipboard">Update from clipboard</NButton>
    </template>
    </NCard>

    <NCard>
        <NInput 
            v-model:value="sextantPriceInput" 
            placeholder="expression" 
            style="max-width: 250px"
            :status="sextantPriceInputStatus"
        >
            <template #prefix>1 sextant=</template>
            <template #suffix>chaos</template>
        </NInput>
    </NCard>

    <NCard>
        {{ netProfitMessage }}
    </NCard>

    <NDataTable
    striped
    :columns="columns" 
    :data="markedData"
    :row-key="row=>row.name"
    v-model:checked-row-keys="checkedRowKeys"
    @update:checked-row-keys="handleCheck"
    />
</template>