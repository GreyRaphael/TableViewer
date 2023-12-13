<template>
    <div id="uppper_container">
        <div>
            <n-dropdown trigger="hover" :options="options" @select="choose_file">
                <n-button :strong="true"> Open </n-button>
            </n-dropdown>
            <n-button @click="clear" :strong="true"> Close </n-button>
        </div>
        <div style="float: right; line-height: 35px; margin: 0 50px;">
            <span>Rows Count: {{ rows_num }}</span>
        </div>
        <div id="query_container">
            <n-button @click="execute_sql" :strong="true"> Execute </n-button>
            <n-input v-model:value="sql" type="text" clearable style="width: 1000px;" />
        </div>
    </div>
    <div id="table_container">
        <n-data-table max-height="calc(100vh - 120px)" :columns="columns" :data="data" size="small"
            style="font-size:smaller;font-weight: 550;" />
    </div>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NInput, NDropdown } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';

let sql = ref("select * from LAST offset 0 limit 100");
const last_filename = ref("");
const rows_num = ref(0);

const options = [
    {
        label: 'parquet',
        key: 'parquet'
    },
    {
        label: 'ipc',
        key: 'arrow'
    }
]

const data = ref([]);
const columns = ref([]);

async function read_parquet_file(filename: string, sql: string) {
    let result: string = await invoke("read_parquet_file", { filename: filename, sql: sql });
    let table = JSON.parse(result);
    columns.value = table["headers"];
    data.value = table["body"];
    rows_num.value = table["row_count"];
}
async function read_ipc_file(filename: string, sql: string) {
    let result: string = await invoke("read_ipc_file", { filename: filename, sql: sql });
    let table = JSON.parse(result);
    columns.value = table["headers"];
    data.value = table["body"];
    rows_num.value = table["row_count"];
}

async function execute_sql() {
    if (sql) {
        if (last_filename.value.endsWith("parquet")) {
            read_parquet_file(last_filename.value, sql.value);
        } else if (last_filename.value.endsWith("arrow")) {
            read_ipc_file(last_filename.value, sql.value);
        }
    }
}

async function choose_file(key: string) {
    const selected = await open({
        multiple: false,
        filters: [{
            name: 'table file',
            extensions: [key,]
        }]
    });

    if (selected && !Array.isArray(selected)) {
        last_filename.value = selected;
        if (key === 'parquet') {
            read_parquet_file(selected, sql.value);
        } else if (key === 'arrow') {
            read_ipc_file(selected, sql.value);
        }

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