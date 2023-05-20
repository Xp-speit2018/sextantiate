<script setup>
import { readText, writeText } from '@tauri-apps/api/clipboard';
import { register, unregister } from '@tauri-apps/api/globalShortcut';

import { useMessage, NButton } from "naive-ui";
import { invoke } from '@tauri-apps/api/tauri';
// import { ref } from 'vue';

window.$message = useMessage();
const message = useMessage();
const sleepInterval = 150;

let isStopped = true;
const mod2key = JSON.parse(localStorage.compassMeta).mod2key

let whitelist = null;
const load_whitelist = () => {
    whitelist = JSON.parse(localStorage.whitelist)
}

const config_screen = {
    sextant: [290, 298],
    voidstone: [493, 671],
    backpack_empty_slot: [545, 580],
    compass_stash: [494, 126],
    currency_stash: [494, 110],
    compass: [107,437]
}

function uses_check(text) {
    const regex = /(\d+) uses remaining/;
    const matches = text.match(regex);
    if (matches[1]!="4") {
        message.error("Compass not 4 uses")
        return false
    }
    return true
}

function parse_mod(text) {
    // console.log(text)
    return text
        .split('\r\n')
        .filter(line=>line.includes("enchant")&&!line.includes("uses remaining"))
        .map(line=>line.replace(" (enchant)", ""))
        .map(line=>line+'\n')
        .sort((a,b)=>a.localeCompare(b))
        .join('')
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}

async function mouse_move_to(pos) {
    return Promise.all([
        invoke("mouse_move_to", {x:pos[0], y:pos[1]}),
        sleep(sleepInterval)
    ])
}

async function mouse_left_click() {
    return Promise.all([
        invoke("mouse_click", {button:"left"}),
        sleep(sleepInterval)
    ])
}

async function mouse_right_click() {
    return Promise.all([
        invoke("mouse_click", {button:"right"}),
        sleep(sleepInterval)
    ])
}

async function quick_shifting() {
    return Promise.all([
        invoke("quick_shifting"),
        sleep(sleepInterval)
    ])
}

async function copy() {
    return Promise.all([
        invoke("copy"),
        sleep(sleepInterval)
    ])
}

async function get_info() {
    await copy();
    // await sleep(20);
    return await readText();
}

async function apply_currency(currency_pos, target_pos) {
    const info = await readText();
    await mouse_move_to(currency_pos);
    if (info == await get_info()) {
        throw "Out of currency"
    }

    await mouse_right_click();
    await mouse_move_to(target_pos);
    await mouse_left_click();
}

async function auto_sextant() {
    load_whitelist();
    while (!isStopped) {
        try {
            console.log('applying sextant to voidstone...');
            await apply_currency(config_screen['sextant'], config_screen['voidstone']);
            const info = await get_info();
            if (!uses_check(info)) throw "Compass not 4 uses";
            const mod = parse_mod(info);
            console.log(mod)
            const key = mod2key[mod];
            if (whitelist.indexOf(key)==-1) continue; // skip undesired mods
            
            // save mod into charged compass
            console.log('saving sextant into charged compass')
            await apply_currency(config_screen['compass'], config_screen['voidstone']);
            await mouse_move_to(config_screen['backpack_empty_slot']);
            await mouse_left_click();
            await mouse_move_to(config_screen['compass_stash']);
            await mouse_left_click();
            await mouse_move_to(config_screen['backpack_empty_slot']);
            await quick_shifting();
            await mouse_move_to(config_screen['currency_stash']);
            await mouse_left_click();
        } catch (error) {
            console.log(error);
            isStopped = true;
            console.log('stopping loop')
            break;
        }
        
    }
}

async function auto_loop() {
    isStopped = !isStopped
    if (!isStopped) {
        console.log("starting sextant loop")
        await writeText('')
        auto_sextant()
    } else {
        console.log("sextant loop stopped")
    }
}

async function onClick() {
    const text = await readText()
    message.info(text)
    const mod  = parse_mod(text)
    const key  = mod2key[mod]
    // console.log(mod)
    message.info(key)
}

async function init() {
    unregister("f3");
    register("f3", auto_loop);
    // register("f3", async ()=>{
    //     if (!isStopped) auto_loop()
    //     console.log(await get_info())
    // })
}

init()

async function uses_check_from_clipboard() {
    const text = await readText();
    uses_check(text);
}

</script>

<template>
sextant Component
<NButton @click="onClick">Try from clipboard</NButton>
<NButton @click="uses_check_from_clipboard">test</NButton>
</template>