<script setup>
import { invoke } from "@tauri-apps/api/tauri"

async function mouse_move_to(x, y) {
    invoke('mouse_move_to', {x:x, y:y})
}

// async function shift_down() {
//     invoke('key_sequence', {s:"f12"})
// }

</script>

<template>
    <h1> Home view page </h1>

    <button @click="()=>mouse_move_to(1000,1000)" >test</button>
</template>