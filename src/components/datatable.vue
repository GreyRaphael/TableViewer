<template>
    <div class="uppper_container">
        <div>
            <n-button @click="update" :strong="true"> Open </n-button>
            <n-button @click="clear" :strong="true"> Close </n-button>
        </div>
        <div>
            <n-auto-complete placeholder="SELECT * FROM CURRENT LIMIT 1000" clearable style="width:1000px;" />
            <n-button @click="clear" :strong="true"> Execute </n-button>
        </div>
    </div>
    <div class="table_container">
        <n-data-table max-height="calc(100vh - 150px)" :columns="columns" :data="data" size="small"
            style="font-size:smaller;font-weight: 550;" :pagination="pagination" />
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NAutoComplete } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive } from 'vue';
import { open } from '@tauri-apps/api/dialog';

const data = ref([]);
const columns = ref([]);
// const pagination = { pageSizes: [10, 100, 200, 500], showSizePicker: true, showQuickJumper: true }
const pagination = reactive({
    pageSize: 100,
    showSizePicker: true,
    showQuickJumper: true,
    pageSizes: [100, 200, 500],
    onUpdatePageSize: (pageSize: number) => {
        pagination.pageSize = pageSize
    }
})


async function get_data(filename: string) {
    let result: string = await invoke("get_data", { filename: filename });
    data.value = JSON.parse(result);
}
async function get_header(filename: string) {
    let result: string = await invoke("get_header", { filename: filename });
    columns.value = JSON.parse(result);
}
async function update() {
    const selected = await open({
        multiple: false,
        filters: [{
            name: 'table file',
            extensions: ['parquet', 'csv', 'json']
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
        get_header(selected);
        get_data(selected);
    }
}

async function clear() {
    data.value = [];
    columns.value = [];
}


</script>

<style>
.uppper_container,
.table_container {
    margin: 5px;
}

.uppper_container>div {
    display: inline-block;
}

.uppper_container>div:nth-child(2) {
    float: right;
}
</style>