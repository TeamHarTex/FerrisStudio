<template>
  <link href='https://fonts.googleapis.com/css?family=Inter:300' rel='stylesheet' type='text/css'>
  <link href='https://fonts.googleapis.com/css?family=Inter:400' rel='stylesheet' type='text/css'>
  <link href='https://fonts.googleapis.com/css?family=Inter:700' rel='stylesheet' type='text/css'>

  <div data-tauri-drag-region class="titlebar">
    <p class="app-title">
      FerrisStudio 2023.2
    </p>
    <div class="buttons">
      <div class="titlebar-button" id="titlebar-minimize" @click="minimize()">
        <img
            src="https://api.iconify.design/fluent/subtract-16-regular.svg?color=white"
            alt="minimize"
        />
      </div>
      <div class="titlebar-button" id="titlebar-maximize" @click="maximize()">
        <img
            src="https://api.iconify.design/fluent/maximize-16-regular.svg?color=white"
            alt="maximize"
        />
      </div>
      <div class="titlebar-button" id="titlebar-close" @click="close()">
        <img src="https://api.iconify.design/fluent/dismiss-16-regular.svg?color=white"
             alt="close"
        />
      </div>
    </div>
  </div>

  <div class="container">
    <h1>Welcome to FerrisStudio</h1>

    <p class="welcome-desc">Create a new project to start from scratch.</p>
    <p class="welcome-desc">Open an existing project from the filesystem or version control system.</p>
  </div>
  <div class="project-buttons">
    <div class="project-button">
      <img class="project-button-image" src="https://api.iconify.design/fluent/add-square-16-filled.svg?width=80&height=80&color=%23007acc" alt="existingProject">
      <br/>
      <p class="button-label">Create New Project</p>
    </div>
    <div class="project-button">
      <img class="existing-project project-button-image" src="https://api.iconify.design/fluent/folder-16-filled.svg?color=%23007acc&width=80&height=80" alt="existingProject">
      <br/>
      <p class="button-label">Open Existing Project</p>
    </div>
    <div class="project-button">
      <img class="get-vcs project-button-image" src="https://api.iconify.design/fluent/branch-16-filled.svg?color=%23007acc&width=80&height=80" alt="existingProject">
      <br/>
      <p class="button-label">Get from VCS</p>
    </div>
  </div>
</template>

<script lang="ts">
import { appWindow } from '@tauri-apps/api/window';

export default {
  methods: {
    async close() {
      await appWindow.close();
    },
    async maximize() {
      await appWindow.toggleMaximize();
    },
    async minimize() {
      await appWindow.minimize();
    }
  }
};
</script>

<style>
body {
  background-color: #3D3F43;
}

h1 {
  color: #FFFFFF;

  margin-top: 15%;

  font-family: Inter, serif;
  font-size: 2.25rem;
  font-weight: 700;
  text-align: center;
}

p {
  font-family: Inter, serif;
}

.app-title {
  color: #A6A6A6;
  height: 50px;
  padding-left: 15px;

  line-height: 20px;
  text-align: center;
}

.titlebar {
  height: 50px;
  background: #242628;
  user-select: none;
  display: flex;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}

.button-label {
  margin-top: -65px;

  color: #FFFFFF;
  font-size: 0.85rem;
  font-weight: 300;
  text-align: center;
}

.buttons {
  margin-left: auto;
}

.project-button {
  display: flex;
  flex-direction: column;
  justify-content: center;

  width: 180px;
}

.project-button-image {
  margin: 50px;
  object-fit: cover;
}

.existing-project {
  background: #B3E0FFBB;
  border-radius: 10px;
  border: solid #B3E0FF00;
  margin: 60px;
}

.get-vcs {
  background: #B3E0FFBB;
  border-radius: 10px;
  border: solid #B3E0FF00;
  margin: 60px;
}

.project-buttons {
  display: flex;
  flex-direction: row;
  justify-content: center;
  margin-top: 75px;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 50px;
  height: 50px;
}

.titlebar-button:hover {
  background: #494B50;
}

#titlebar-close:hover {
  background: #FF4D4D;
}

.welcome-desc {
  color: #C9CBCF;
  font-size: .8rem;
  font-weight: 400;
  text-align: center;

  margin-top: 0;
}
</style>
