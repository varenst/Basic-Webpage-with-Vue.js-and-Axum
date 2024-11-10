<script setup lang="ts">
import { ref } from 'vue';

const selectedFile = ref<File | null>(null);
const result = ref<string | null>(null);
const imagePreview = ref<string | null>(null);
const loading = ref(false);

function onFileSelected(event: Event) {
  if (loading.value) return; 

  const target = event.target as HTMLInputElement;
  if (target.files && target.files.length > 0) {
    selectedFile.value = target.files[0];
    imagePreview.value = URL.createObjectURL(selectedFile.value);
  }
}

async function onUpload() {
  if (!selectedFile.value || loading.value) return;

  loading.value = true; 
  result.value = null; 

  const formData = new FormData();
  formData.append("file", selectedFile.value);

  try {
    const response = await fetch("http://localhost:3000/upload", {
      method: "POST",
      body: formData,
    });
    result.value = await response.text();
  } catch (error) {
    result.value = "Error during upload, please try again.";
  } finally {
    loading.value = false; 
  }
}
</script>

<template>
  <div class="upload-container">
    <h1>Image Classifier</h1>

    <div class="upload-controls">
      <input type="file" @change="onFileSelected" accept="image/*" :disabled="loading" />
      <button @click="onUpload" :disabled="!selectedFile || loading">
        {{ loading ? "Uploading..." : "Upload" }}
      </button>
    </div>

    <div v-if="imagePreview" class="preview">
      <h3>Selected Image:</h3>
      <img :src="imagePreview" alt="Image Preview" />
    </div>

    <div v-if="result" class="result">
      <h2>Classification Result:</h2>
      <p>{{ result }}</p>
    </div>
  </div>
</template>

<style scoped>
.upload-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  font-family: Arial, sans-serif;
  padding: 20px;
  max-width: 600px;
  margin: auto;
}

h1 {
  color: #333;
  margin-bottom: 20px;
}

.upload-controls {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

button {
  padding: 10px 20px;
  background-color: #4caf50;
  color: white;
  border: none;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: #45a049;
}

.preview {
  margin: 20px 0;
  text-align: center;
}

.preview img {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
  box-shadow: 0px 4px 8px rgba(0, 0, 0, 0.1);
}

.result {
  margin-top: 20px;
  text-align: center;
}

.result p {
  font-size: 1.2em;
  color: #0073e6;
  font-weight: bold;
}
</style>
