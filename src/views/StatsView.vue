<script setup>
import { NCard, NButton, useMessage, NTransfer, NCode } from "naive-ui"
import { ref, computed } from "vue"
import { fetch } from "@tauri-apps/api/http"
import debounce from "@/utils/debounce";

window.$message = useMessage()
const message = useMessage()

const title = ref('My stash tab')

const tabs = ref([])
const selected = ref([])
const compassesInStash = ref([])
const sortedCompassInStash = computed(()=>{
    const tmp = [...compassesInStash.value]
    return tmp.sort((a,b)=>b.price-a.price)
})

if (localStorage.tabs && localStorage.selected) {
    tabs.value = JSON.parse(localStorage.tabs)
    selected.value = JSON.parse(localStorage.selected)
}

const mod2key = JSON.parse(localStorage.compassMeta).mod2key
const price = JSON.parse(localStorage.price).data
const config_account = JSON.parse(localStorage.config_account)
const priceMap = {}
price.map(item => {
    priceMap[item.name] = item.chaos
})

const reportText = computed(()=>{
    let text = `Wts Softcore compasses (4 uses) | ign : ${config_account.IGN}\n`
    sortedCompassInStash.value.map(item => {
        text += `${item.count}x ${item.name} ${item.price}c/ea\n`
    })
    console.log(text)
    return text
})

const reportSum = computed(()=>{
    let sum = 0
    sortedCompassInStash.value.map(item => {
        sum += item.count*item.price
    })
    return sum
})

// const compassesPriced = computed(()=>{
//     return compassesInStash.value
// })

const save = debounce(()=>{
    localStorage.tabs = JSON.stringify(tabs.value)
    localStorage.selected = JSON.stringify(selected.value)
    message.success('saved')
}, 2000)

const fetchListLoading = ref(false)
const fetchList = async () => {
    if (!localStorage.config_account) {
        message.info('Navigate to config panel to provide account information')
        return
    }
    fetchListLoading.value = true
    try {
        const config = JSON.parse(localStorage.config_account)
        const url = `https://www.pathofexile.com/character-window/get-stash-items?accountName=${config.accountName}&realm=${config.realm}&league=${config.league}&tabs=1&tabIndex=0`
        const res = await fetch(url, {headers:{cookie:"POESESSID="+config.POESESSID}})
        tabs.value = res.data.tabs.map(tab => { return {value:tab.i, label:tab.n} })
        selected.value = []
        save()
    } catch (error) {
        message.error(error)
    } finally {
        fetchListLoading.value = false
    }
}

const fetchStashesLoading = ref(false)
const fetchStashes = async() => {
    fetchStashesLoading.value = true
    try {
        const config = JSON.parse(localStorage.config_account)
        let itemsAgg = []
        for (let id of selected.value) {
            let url = `https://www.pathofexile.com/character-window/get-stash-items?accountName=${config.accountName}&realm=${config.realm}&league=${config.league}&tabs=0&tabIndex=${id}`
            let res = await fetch(url, {headers:{cookie:"POESESSID="+config.POESESSID}})
            let items = res.data.items
                .filter(item=>item.baseType=="Charged Compass")
                .map(item=>item.enchantMods
                    .filter(line=>!line.includes("uses remaining"))
                    .map(item=>item+'\n')
                    .sort((a,b)=>a.localeCompare(b))
                    .join('')
                )
                .map(item=>mod2key[item])

            
            
            items.map(item=>{
                const id = itemsAgg.findIndex(compass=>compass.name==item)
                if (id==-1) {
                    itemsAgg.push({
                        name: item,
                        count: 1,
                        price: priceMap[item]
                    })
                } else {
                    itemsAgg[id].count += 1
                }
            })
        }
        console.log(itemsAgg)
        compassesInStash.value = itemsAgg
    } catch (error) {
        message.error(error)
    }
    fetchStashesLoading.value = false
}


</script>

<template>
    <NCard :title="title" embedded hoverable>
    <template #header-extra>

        <NButton :loading="fetchListLoading" @click="fetchList"> Fetch Stash Tab List </NButton>
        <NButton :loading="fetchStashesLoading" @click="fetchStashes"> Fetch Stashes </NButton>
    </template>
    </NCard>
    <NTransfer
        v-model:options="tabs"
        v-model:value="selected"
        @update:value="save"
    />
    <NCard> Total: {{ reportSum }} chaos </NCard>
    <NCode :code="reportText" show-line-numbers style="text-align: left"/>
</template>