<script setup lnag="ts">
import { reactive } from 'vue';

let { data: user_data, error } = await useFetch('/api/user');

console.log('d', user_data, error);

let { total: user_total, records: user_records } = user_data.value;

let form = reactive({
  title: 'hello',
  member: [],
});
</script>

<template>
  <div>
    <div class="card">
      <input v-model="form.title" placeholder="프로젝트 이름"/>
      <Listbox v-model="form.member" :list="user_records" multiple>
        <template #select="{ item: user }">
          {{ `${user.name} (${user.username})` }}
        </template>
        <template #list="{ item: user }">
          {{ `${user.name} (${user.username})` }}
        </template>
      </Listbox>
      <button>저장</button>
    </div>
  </div>
</template>