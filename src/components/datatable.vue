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
            <n-tooltip placement="bottom" trigger="hover">
                <template #trigger>
                    <n-input v-model:value="sql" type="text" clearable style="width: 1000px;"
                        placeholder="select * from LAST where idx>10 order by idx desc offset 0 limit 100" />
                </template>
                <span>Not support field with "-"</span>
            </n-tooltip>
        </div>
    </div>
    <div id="table_container">
        <n-data-table max-height="calc(100vh - 120px)" :columns="tb_headers" :data="tb_body" size="small"
            style="font-size:smaller;font-weight: 550;" />
    </div>
    <n-modal v-model:show="showModal" preset="dialog" negative-text="Cancel" positive-text="Ok"
        @positive-click="config_csv" title="Choose Separator">
        <n-select v-model:value="csv_sep" :options="csv_sep_combos" />
    </n-modal>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NInput, NDropdown, NSelect, NModal, NTooltip } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, ref } from 'vue';
import { open, message } from '@tauri-apps/api/dialog';

// parameters
const sql = ref("");
const last_filenames: Ref<string[]> = ref([]);
const last_filetype = ref("");
// table headers & body
const tb_cols = ref(0);
const tb_rows = ref(0);
const tb_headers = ref([]);
const tb_body = ref([]);

const filetype_options = [
    {
        label: 'parquet(s)',
        key: 'parquet'
    },
    {
        label: 'feather(s)',
        key: 'arrow'
    },
    {
        label: 'csv(s)',
        key: 'csv'
    }
]

async function set_ui(j_str: string) {
    let table = JSON.parse(j_str);
    if (table.hasOwnProperty("err_msg")) {
        message(table["err_msg"], { title: 'sql query error', type: 'error' });
    } else {
        tb_headers.value = table["headers"];
        tb_body.value = table["body"];
        tb_rows.value = table["row_count"];
        tb_cols.value = table["col_count"];
    }
}

async function read_parquet_files(filelist: string[], sql: string) {
    let filenames = JSON.stringify(filelist);
    let result: string = await invoke("read_parquet_files", { filenames: filenames, sql: sql });
    set_ui(result);
}
async function read_ipc_files(filelist: string[], sql: string) {
    let filenames = JSON.stringify(filelist);
    let result: string = await invoke("read_ipc_files", { filenames: filenames, sql: sql });
    set_ui(result);
}
async function read_csv_files(filelist: string[], sql: string, sep: number) {
    let filenames = JSON.stringify(filelist);
    let result: string = await invoke("read_csv_files", { filenames: filenames, sql: sql, sep: sep });
    set_ui(result);
}

async function execute_sql() {
    if (sql) {
        if ('parquet' == last_filetype.value) {
            read_parquet_files(last_filenames.value, sql.value);
        } else if ('arrow' == last_filetype.value) {
            read_ipc_files(last_filenames.value, sql.value);
        } else if ('csv' == last_filetype.value) {
            read_csv_files(last_filenames.value, sql.value, csv_sep.value.charCodeAt(0));
        }
    }
}

interface ExtensionMapType {
    [key: string]: string[]; // Index signature: any string key maps to an array of strings
}

const Extensions: ExtensionMapType = {
    "parquet": ["parquet",],
    "arrow": ["arrow", "ipc", "feather"],
    "csv": ["csv",],
};

async function choose_filetype(key: string) {
    sql.value = "select * from LAST offset 0 limit 100";
    last_filetype.value = key;
    const selected = await open({
        multiple: true,
        filters: [{
            name: 'Table File(s)',
            extensions: Extensions[key]
        }, {
            name: 'All File(s)',
            extensions: ["*",]
        }]
    });
    // console.log(selected);

    if (Array.isArray(selected)) {
        // user selected multiple files
        last_filenames.value = selected;
        if ('parquet' == key) {
            read_parquet_files(last_filenames.value, sql.value);
        } else if ('arrow' == key) {
            read_ipc_files(last_filenames.value, sql.value);
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
    read_csv_files(last_filenames.value, sql.value, csv_sep.value.charCodeAt(0));
}

// modal
const showModal = ref(false);
const csv_sep = ref(',');
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