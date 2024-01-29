<script setup lang="ts" generic="T">
import { computed } from 'vue';

let props = defineProps({
  modelValue: {},
  list: Array<T>,
  iden: {
    type: String,
    default: 'id',
  },
  multiple: Boolean,
});

let emits = defineEmits(['update:modelValue']);

let update = (value: T) => emits('update:modelValue', value);

let selected = computed(() => {
  if(props.multiple) {
    return props.modelValue?.map(val => props.list?.find(item => item[props.iden] == val));
  } else {
    return props.list?.find(item => item[props.iden] == props.modelValue);
  }
});
</script>

<template>
  <HeadlessListbox :modelValue="modelValue" @update:modelValue="update" :multiple="multiple">
    <div class="listbox-wrapper">
      <HeadlessListboxButton class="listbox-button">
        <template v-if="multiple">
          <div v-for="item, idx in selected">
            <slot name="select" v-bind="{ item, idx }"/>
          </div>
        </template>
        <template v-else>
          <slot name="select" v-bind="{ item: selected, idx: null }"/>
        </template>
      </HeadlessListboxButton>
      <HeadlessListboxOptions class="listbox-options">
        <HeadlessListboxOption v-for="item, idx in list" :value="item[iden]">
          <slot name="list" v-bind="{ item, idx }"/>
        </HeadlessListboxOption>
      </HeadlessListboxOptions>
    </div>
  </HeadlessListBox>
</template>

<style scoped lang="scss">
@use '~/assets/color.scss' as *;
.listbox-wrapper {
  position: relative;

  .listbox-button {
    width: 100%;
    color: $zinc-900;
    background-color: $zinc-50;
  }

  .listbox-options {
    position: absolute;
  }
}
</style>