<template>
    <div id="uppper_container">
        <div>
            <n-dropdown trigger="hover" size="small" :options="options" @select="handleSelect">
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
        <n-data-table max-height="calc(100vh - 150px)" :columns="columns" :data="data" size="small"
            style="font-size:smaller;font-weight: 550;" :pagination="pagination" />
    </div>
    <n-modal v-model:show="showModal" preset="dialog" negative-text="Cancel" positive-text="Ok"
        @positive-click="submitCallback" title="Choose Separator">
        <n-select v-model:value="combo_value" :options="combo_options" />
    </n-modal>
</template>

<script setup lang="ts">
import { NButton, NDataTable, NAutoComplete, NModal, NDropdown, NSelect } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive } from 'vue';
import { open } from '@tauri-apps/api/dialog';

const showModal = ref(false);

async function submitCallback() {
    choose_file("csv");
}


const options = [
    {
        label: 'parquet',
        key: 'parquet'
    },
    {
        label: 'csv',
        key: 'csv'
    },
    {
        label: 'json',
        key: 'json'
    }
]

async function handleSelect(key: string) {
    if ('csv' == key) {
        showModal.value = true;
    } else {
        choose_file(key)
    }
}

const combo_value = ref(';');

const combo_options = [
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
    },
    {
        label: "space( )",
        value: ' ',
    }
]

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

async function read_file(filename: string) {
    let result: string = await invoke("read_file", { filename: filename });
    let table = JSON.parse(result);
    columns.value = table["headers"];
    data.value = table["body"];
}

async function read_csv_table(filename: string, sep: string, limit: number) {
    let result: string = await invoke("read_file", { filename: filename, sep: sep, limit: limit });
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
        if ('csv' == key) {
            read_csv_table(selected, combo_value.value, 1000);
        } else {
            read_file(selected);
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