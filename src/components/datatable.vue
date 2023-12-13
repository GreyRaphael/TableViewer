<template>
    <div id="uppper_container">
        <div>
            <n-dropdown trigger="hover" size="small" :options="options" @select="choose_file">
                <n-button :strong="true"> Open </n-button>
            </n-dropdown>
        </div>
        <div>
            <n-button @click="clear" :strong="true"> Close </n-button>
        </div>
        <div id="query_container">
            <n-auto-complete placeholder="SELECT * FROM CURRENT LIMIT 1000" clearable style="width:1000px;" />
            <n-button :strong="true"> Execute </n-button>
        </div>
    </div>
    <div id="table_container">
        <n-data-table max-height="calc(100vh - 100px)" :columns="columns" :data="data" size="small"
            style="font-size:smaller;font-weight: 550;" />
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NAutoComplete, NDropdown } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';


const options = [
    {
        label: 'parquet',
        key: 'parquet'
    },
    {
        label: 'json',
        key: 'json'
    }
]

const data = ref([]);
const columns = ref([]);

async function read_file(filename: string) {
    let result: string = await invoke("read_file", { filename: filename });
    let table = JSON.parse(result);
    columns.value = table["headers"];
    data.value = table["body"];
}

async function choose_file(key: string) {
    const selected = await open({
        multiple: false,
        filters: [{
            name: 'table file',
            extensions: [key,]
        }]
    });
    if (Array.isArray(selected)) {
        // user selected multiple files
        console.log(selected);
    } else if (selected === null) {
        // user cancelled the selection
        console.log('cancel');
    } else {
        // user selected a single file
        read_file(selected);
    }
}

async function clear() {
    data.value = [];
    columns.value = [];
}


</script>

<style>
#uppper_container,
#table_container {
    margin: 5px;
}

#uppper_container>div {
    display: inline-block;
}

#query_container {
    float: right;
}
</style>