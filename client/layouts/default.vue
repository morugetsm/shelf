<script setup lang="ts">
import { SunIcon, MoonIcon, ComputerDesktopIcon } from '@heroicons/vue/20/solid';
import { UserCircleIcon } from '@heroicons/vue/24/outline';

let colorMode = useColorMode();

let changeTheme = () => {
  switch(colorMode.preference) {
    case 'light':
      colorMode.preference = 'dark';
      break;
    case 'dark':
      colorMode.preference = 'system';
      break;
    case 'system':
      colorMode.preference = 'light';
      break;
  }
};
</script>

<template>
  <div>
    <header>
      <div class="header-wrapper container mx-auto">
        <div class="header-left">
          <NuxtLink to="/">Shelf</NuxtLink>
        </div>
        <ul class="header-center">
          <li>
            <NuxtLink to="/notice">Notice</NuxtLink>
          </li>
          <li>
            <NuxtLink to="/new_project">Project</NuxtLink>
          </li>
        </ul>
        <div class="header-right">
          <SunIcon class="theme" @click="changeTheme" v-if="colorMode.preference == 'light'"/>
          <MoonIcon class="theme" @click="changeTheme" v-else-if="colorMode.preference == 'dark'"/>
          <ComputerDesktopIcon class="theme" @click="changeTheme" v-else/>
          <UserCircleIcon class="user"/>
        </div>
      </div>
    </header>
    <main>
      <div class="main-wrapper container mx-auto">
        <slot/>
      </div>
    </main>
  </div>
</template>

<style scoped lang="scss">
@use "sass:color";
@use "~/assets/color.scss" as *;

header {
  width: 100%;
  height: 5rem;
  position: sticky;
  top: 0;
  padding-top: 1rem;

  div.header-wrapper {
    height: 4rem;
    padding: 0 1rem 0 1.25rem;
    background-color: color.adjust($teal-200, $alpha: -0.6);
    backdrop-filter: blur(0.5rem);

    border-radius: 2rem;

    display: flex;
    flex-flow: row nowrap;
    align-items: center;
    justify-content: space-between;

    div.header-left {
      font-size: 1.5rem;
    }
    ul.header-center {
      display: flex;
      flex-flow: row nowrap;
      gap: 0.5rem;
      li {
        padding: 0.125rem 0.5rem;
        border-radius: 0.25rem;
        cursor: pointer;

        &:hover {
          background-color: color.adjust($teal-200, $alpha: -0.6);
        }
      }
    }
  
    div.header-right {
      display: flex;
      flex-flow: row nowrap;
      align-items: center;
      gap: 0.5rem;
      .theme {
        width: 1.25rem;
        height: 1.25rem;
      }
      .user {
        width: 2.5rem;
        height: 2.5rem;
      }
    }
  }
}

main {
  div.main-wrapper {
    padding: 1.5rem;
  }
}
</style>
