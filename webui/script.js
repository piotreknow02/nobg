const uploadArea = document.getElementById('upload-area');
const fileInput = document.getElementById('file-input');
const modelSelect = document.getElementById('model-select');
const previewContainer = document.getElementById('preview-container');
const previewImage = document.getElementById('preview-image');
const loading = document.getElementById('loading');
const errorDiv = document.getElementById('error');
const resultWrapper = document.getElementById('result-wrapper');
const resultImage = document.getElementById('result-image');
const resultButtons = document.getElementById('result-buttons');
const downloadBtn = document.getElementById('download-btn');
const resetBtn = document.getElementById('reset-btn');
const processBtnContainer = document.getElementById('process-btn-container');
const processBtn = document.getElementById('process-btn');

let currentFile = null;

async function loadModels() {
    try {
        const response = await fetch('/api/models');
        const models = await response.json();
        
        modelSelect.innerHTML = '';
        
        const downloaded = models.filter(m => m.downloaded);
        const notDownloaded = models.filter(m => !m.downloaded);
        
        if (downloaded.length > 0) {
            const optgroupDownloaded = document.createElement('optgroup');
            optgroupDownloaded.label = 'Downloaded';
            downloaded.forEach(m => {
                const option = document.createElement('option');
                option.value = m.name;
                option.textContent = m.name;
                optgroupDownloaded.appendChild(option);
            });
            modelSelect.appendChild(optgroupDownloaded);
        }
        
        if (notDownloaded.length > 0) {
            const optgroupAvailable = document.createElement('optgroup');
            optgroupAvailable.label = 'Available (not downloaded)';
            notDownloaded.forEach(m => {
                const option = document.createElement('option');
                option.value = m.name;
                option.textContent = m.name + ' (not downloaded)';
                option.disabled = true;
                optgroupAvailable.appendChild(option);
            });
            modelSelect.appendChild(optgroupAvailable);
        }
        
        if (downloaded.length === 0) {
            modelSelect.innerHTML = '<option value="">No models downloaded</option>';
        }
    } catch (err) {
        console.error('Failed to load models:', err);
        modelSelect.innerHTML = '<option value="">Failed to load models</option>';
    }
}

loadModels();

uploadArea.addEventListener('click', () => fileInput.click());

uploadArea.addEventListener('dragover', (e) => {
    e.preventDefault();
    uploadArea.classList.add('dragover');
});

uploadArea.addEventListener('dragleave', () => {
    uploadArea.classList.remove('dragover');
});

uploadArea.addEventListener('drop', (e) => {
    e.preventDefault();
    uploadArea.classList.remove('dragover');
    const files = e.dataTransfer.files;
    if (files.length > 0) {
        handleFile(files[0]);
    }
});

fileInput.addEventListener('change', (e) => {
    if (e.target.files.length > 0) {
        handleFile(e.target.files[0]);
    }
});

function handleFile(file) {
    if (!file.type.startsWith('image/')) {
        showError('Please upload an image file');
        return;
    }

    currentFile = file;
    const reader = new FileReader();
    reader.onload = (e) => {
        previewImage.src = e.target.result;
        previewContainer.style.display = 'block';
        resultWrapper.style.display = 'none';
        resultButtons.style.display = 'none';
        processBtnContainer.style.display = 'block';
        hideError();
    };
    reader.readAsDataURL(file);
}

async function processImage() {
    if (!currentFile) {
        showError('Please upload an image first');
        return;
    }

    const selectedModel = modelSelect.value;
    if (!selectedModel) {
        showError('Please select a model');
        return;
    }

    loading.style.display = 'block';
    hideError();
    resultWrapper.style.display = 'none';
    resultButtons.style.display = 'none';
    processBtnContainer.style.display = 'none';

    const formData = new FormData();
    formData.append('image', currentFile);
    formData.append('model', selectedModel);

    try {
        const response = await fetch('/api/remove-bg', {
            method: 'POST',
            body: formData
        });

        const data = await response.json();

        if (data.success) {
            resultImage.src = `data:image/png;base64,${data.image}`;
            downloadBtn.href = `data:image/png;base64,${data.image}`;
            resultWrapper.style.display = 'block';
            resultButtons.style.display = 'flex';
        } else {
            showError(data.error || 'Failed to process image');
        }
    } catch (err) {
        showError('Network error: ' + err.message);
    } finally {
        loading.style.display = 'none';
    }
}

function showError(msg) {
    errorDiv.textContent = msg;
    errorDiv.style.display = 'block';
}

function hideError() {
    errorDiv.style.display = 'none';
}

function reset() {
    currentFile = null;
    fileInput.value = '';
    previewContainer.style.display = 'none';
    resultWrapper.style.display = 'none';
    resultButtons.style.display = 'none';
    processBtnContainer.style.display = 'none';
    hideError();
}

document.addEventListener('keydown', (e) => {
    if (e.key === 'Enter' && currentFile && resultWrapper.style.display === 'none') {
        processImage();
    }
});

let processTimeout = null;
uploadArea.addEventListener('mouseup', () => {
    if (currentFile && previewContainer.style.display === 'block' && resultWrapper.style.display === 'none') {
        processTimeout = setTimeout(processImage, 300);
    }
});

resetBtn.addEventListener('click', reset);

if (processBtn) {
    processBtn.addEventListener('click', processImage);
}
