<template>
    <div id="uppper_container">
        <div>
            <n-dropdown trigger="hover" :options="filetype_options" @select="choose_filetype">
                <n-button :strong="true"> Open </n-button>
            </n-dropdown>
            <n-button @click="clear" :strong="true"> Close </n-button>
        </div>
        <div style="float: right; line-height: 35px; margin: 0 50px;">
            <span>Shape: ({{ tb_rows }}, {{ tb_cols }})</span>
        </div>
        <div id="query_container">
            <n-button @click="execute_sql" :strong="true"> Execute </n-button>
            <n-input v-model:value="sql" type="text" clearable style="width: 1000px;" />
        </div>
    </div>
    <div id="table_container">
        <n-data-table max-height="calc(100vh - 120px)" :columns="tb_headers" :data="tb_body" size="small"
            style="font-size:smaller;font-weight: 550;" />
    </div>
    <n-modal v-model:show="showModal" preset="dialog" negative-text="Cancel" positive-text="Ok" @positive-click="config_csv"
        title="Choose Separator">
        <n-select v-model:value="csv_sep" :options="csv_sep_combos" />
    </n-modal>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NInput, NDropdown, NSelect, NModal } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from 'vue';
import { open } from '@tauri-apps/api/dialog';

// parameters
const sql = ref("select * from LAST offset 0 limit 100");
const last_filename = ref("");
// table headers & body
const tb_cols = ref(0);
const tb_rows = ref(0);
const tb_headers = ref([]);
const tb_body = ref([]);

const filetype_options = [
    {
        label: 'parquet',
        key: 'parquet'
    },
    {
        label: 'arrow',
        key: 'arrow'
    },
    {
        label: 'csv',
        key: 'csv'
    }
]


async function read_parquet_file(filename: string, sql: string) {
    let result: string = await invoke("read_parquet_file", { filename: filename, sql: sql });
    let table = JSON.parse(result);
    tb_headers.value = table["headers"];
    tb_body.value = table["body"];
    tb_rows.value = table["row_count"];
    tb_cols.value = table["col_count"];
}
async function read_ipc_file(filename: string, sql: string) {
    let result: string = await invoke("read_ipc_file", { filename: filename, sql: sql });
    let table = JSON.parse(result);
    tb_headers.value = table["headers"];
    tb_body.value = table["body"];
    tb_rows.value = table["row_count"];
    tb_cols.value = table["col_count"];
}
async function read_csv_file(filename: string, sql: string, sep: number) {
    let result: string = await invoke("read_csv_file", { filename: filename, sql: sql, sep: sep });
    let table = JSON.parse(result);
    tb_headers.value = table["headers"];
    tb_body.value = table["body"];
    tb_rows.value = table["row_count"];
    tb_cols.value = table["col_count"];
}

async function execute_sql() {
    if (sql) {
        if (last_filename.value.endsWith("parquet")) {
            read_parquet_file(last_filename.value, sql.value);
        } else if (last_filename.value.endsWith("arrow")) {
            read_ipc_file(last_filename.value, sql.value);
        } else if (last_filename.value.endsWith("csv")) {
            read_csv_file(last_filename.value, sql.value, csv_sep.value.charCodeAt(0));
        }
    }
}

async function choose_filetype(key: string) {
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
        } else if ('csv' == key) {
            showModal.value = true;
        }

    }
}

async function clear() {
    tb_rows.value = 0;
    tb_body.value = [];
    tb_headers.value = [];
}

async function config_csv() {
    read_csv_file(last_filename.value, sql.value, csv_sep.value.charCodeAt(0));
}

// modal
const showModal = ref(false);
const csv_sep = ref(';');
const csv_sep_combos = [
    {
        label: "comma(,)",
        value: ',',
    },
    {
        label: "tab(\\t)",
        value: '\t',
    },
    {
        label: "semicolon(;)",
        value: ';',
    }
]
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