<script setup lang="ts">
import { ref } from 'vue';

const message = ref(null);
const selectedFile = ref<File | null>(null);
const result = ref<string | null>(null); 

async function getMessage() {
  const response = await fetch('http://localhost:3000/');
  const json = await response.json();
  message.value = json.message;
}

function onFileSelected(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    selectedFile.value = target.files[0];
  }
}

async function onUpload() {
  if (!selectedFile.value) return;

  const formData = new FormData();
  formData.append("file", selectedFile.value); 

  const response = await fetch("http://localhost:3000/upload", {
    method: "POST",
    body: formData,
  });
  result.value = await response.text(); 
}

</script>

<template>
  <h1>Rust + Vue</h1>

  <div v-if="message">
    Received: {{ message }}
  </div>
  <div v-else>
    No message
  </div>
  <button @click="getMessage">Get message</button>

  <div class="upload">
    <input type="file" @change="onFileSelected">
    <button @click="onUpload" :disabled="!selectedFile">Upload</button>
  </div>

  <div v-if="result">
    <h2>Classification Result:</h2>
    <p>{{ result }}</p>
  </div>
</template>
