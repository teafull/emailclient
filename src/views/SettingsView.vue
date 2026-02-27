<script setup>
import { ref, watch, onMounted, nextTick } from "vue";

const props = defineProps({
  labels: {
    type: Array,
    default: () => []
  },
  settings: {
    type: Object,
    default: () => ({})
  }
});

const emit = defineEmits(["update:labels", "update:settings"]);

const defaultLlm = {
  enabled: true,
  provider: "OpenAI",
  model: "gpt-4o-mini",
  apiKey: "",
  endpoint: "",
  tone: "专业"
};

const buildSettings = (value) => ({
  ...value,
  llm: { ...defaultLlm, ...(value?.llm || {}) }
});

const localLabels = ref(Array.isArray(props.labels) ? [...props.labels] : []);
const localSettings = ref(buildSettings(props.settings));
const signatureRef = ref(null);
const providerOptions = ["OpenAI", "DeepSeek", "Qwen", "Claude", "Azure OpenAI"];


const textToHtml = (value) =>
  (value || "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;")
    .replace(/\n/g, "<br />");

const syncSignatureEditor = async () => {
  await nextTick();
  if (!signatureRef.value) {
    return;
  }
  const html = localSettings.value.signatureHtml || textToHtml(localSettings.value.signature);
  if (signatureRef.value.innerHTML !== html) {
    signatureRef.value.innerHTML = html || "";
  }
};

watch(
  () => props.labels,
  (value) => {
    localLabels.value = Array.isArray(value) ? [...value] : [];
  },
  { deep: true }
);

watch(
  () => props.settings,
  (value) => {
    localSettings.value = buildSettings(value);
    if (!localSettings.value.signatureHtml && localSettings.value.signature) {
      localSettings.value.signatureHtml = textToHtml(localSettings.value.signature);
    }
    syncSignatureEditor();
  },
  { deep: true }
);


watch(
  localLabels,
  (value) => {
    emit("update:labels", [...value]);
  },
  { deep: true }
);

watch(
  localSettings,
  (value) => {
    emit("update:settings", { ...value });
  },
  { deep: true }
);

onMounted(() => {
  syncSignatureEditor();
});

const newLabel = ref("");

const addLabel = () => {
  const value = newLabel.value.trim();
  if (!value) {
    return;
  }
  if (localLabels.value.includes(value)) {
    return;
  }
  localLabels.value.push(value);
  newLabel.value = "";
};

const removeLabel = (label) => {
  localLabels.value = localLabels.value.filter((item) => item !== label);
};

const updateSignature = () => {
  if (!signatureRef.value) {
    return;
  }
  localSettings.value.signatureHtml = signatureRef.value.innerHTML;
};

const applyFormat = (tag) => {
  if (!signatureRef.value) {
    return;
  }
  signatureRef.value.focus();
  const selection = window.getSelection();
  if (!selection || !selection.rangeCount) {
    return;
  }
  const range = selection.getRangeAt(0);
  const isCollapsed = selection.isCollapsed;
  const wrapper = document.createElement(tag);

  if (isCollapsed) {
    wrapper.innerHTML = "&nbsp;";
    range.insertNode(wrapper);
    const newRange = document.createRange();
    newRange.setStart(wrapper.firstChild, 1);
    newRange.collapse(true);
    selection.removeAllRanges();
    selection.addRange(newRange);
  } else {
    const contents = range.extractContents();
    wrapper.appendChild(contents);
    range.insertNode(wrapper);
    selection.removeAllRanges();
    const newRange = document.createRange();
    newRange.selectNodeContents(wrapper);
    newRange.collapse(false);
    selection.addRange(newRange);
  }
  updateSignature();
};


const insertImage = (src) => {
  if (!signatureRef.value) {
    return;
  }
  signatureRef.value.focus();
  const selection = window.getSelection();
  if (selection && selection.rangeCount) {
    const range = selection.getRangeAt(0);
    const image = document.createElement("img");
    image.src = src;
    image.alt = "签名图片";
    image.style.maxWidth = "120px";
    image.style.verticalAlign = "middle";
    range.insertNode(image);
    range.collapse(false);
    selection.removeAllRanges();
    selection.addRange(range);
  } else {
    signatureRef.value.insertAdjacentHTML("beforeend", `<img src="${src}" alt="签名图片" />`);
  }
  updateSignature();
};

const handleImageSelect = (event) => {
  const file = event.target.files?.[0];
  if (!file) {
    return;
  }
  const reader = new FileReader();
  reader.onload = () => {
    insertImage(reader.result);
    event.target.value = "";
  };
  reader.readAsDataURL(file);
};

const themeOptions = ["专业蓝", "深邃蓝", "极简白"];
</script>

<template>
  <div class="settings-panel">
    <div class="settings-header">
      <div class="settings-title">设置</div>
      <div class="settings-subtitle">管理账号信息、通知与写信偏好</div>
    </div>

    <el-card shadow="never" class="settings-card">
      <div class="section-title">账号信息</div>
      <el-form :model="localSettings" label-width="90px" class="settings-form">
        <el-form-item label="显示名称">
          <el-input v-model="localSettings.displayName" placeholder="请输入显示名称" />
        </el-form-item>
        <el-form-item label="职位">
          <el-input v-model="localSettings.title" placeholder="请输入职位" />
        </el-form-item>
        <el-form-item label="邮箱">
          <el-input v-model="localSettings.email" placeholder="请输入邮箱" />
        </el-form-item>
      </el-form>
    </el-card>

    <el-card shadow="never" class="settings-card settings-card--wide">
      <div class="section-title">写信偏好</div>

      <el-form :model="localSettings" label-width="100px" class="settings-form">
        <el-form-item label="默认签名">
          <div class="signature-editor">
            <div class="signature-toolbar">
              <el-button size="small" @click="applyFormat('bold')">加粗</el-button>
              <el-button size="small" @click="applyFormat('italic')">斜体</el-button>
              <el-button size="small" @click="applyFormat('underline')">下划线</el-button>
              <label class="upload-btn">
                插入图片
                <input type="file" accept="image/*" @change="handleImageSelect" />
              </label>
            </div>
            <div
              ref="signatureRef"
              class="signature-input"
              contenteditable="true"
              data-placeholder="请输入签名内容..."
              @input="updateSignature"
              @blur="updateSignature"
            ></div>
            <div class="signature-tip">支持富文本与图片，保存后写邮件自动带出。</div>
          </div>
        </el-form-item>
        <el-form-item label="自动保存草稿">
          <el-switch v-model="localSettings.autoDraft" />
        </el-form-item>
      </el-form>
    </el-card>

    <el-card shadow="never" class="settings-card">
      <div class="section-title">大模型配置</div>
      <el-form :model="localSettings" label-width="90px" class="settings-form">
        <el-form-item label="启用">
          <el-switch v-model="localSettings.llm.enabled" />
        </el-form-item>
        <el-form-item label="服务商">
          <el-select v-model="localSettings.llm.provider" placeholder="请选择服务商" style="width: 200px">
            <el-option v-for="item in providerOptions" :key="item" :label="item" :value="item" />
          </el-select>
        </el-form-item>
        <el-form-item label="模型">
          <el-input v-model="localSettings.llm.model" placeholder="例如 gpt-4o-mini" />
        </el-form-item>
        <el-form-item label="API Key">
          <el-input v-model="localSettings.llm.apiKey" type="password" show-password placeholder="请输入 API Key" />
        </el-form-item>
        <el-form-item label="Endpoint">
          <el-input v-model="localSettings.llm.endpoint" placeholder="可选，自定义接入地址" />
        </el-form-item>
        <el-form-item label="写作风格">
          <el-input v-model="localSettings.llm.tone" placeholder="例如：专业、简洁、礼貌" />
        </el-form-item>
      </el-form>
      <div class="settings-tip">用于在写邮件/回复时生成内容，当前为本地模拟。</div>
    </el-card>

    <el-card shadow="never" class="settings-card">
      <div class="section-title">标签设置</div>

      <div class="label-manager">
        <div class="label-list">
          <el-tag v-for="label in localLabels" :key="label" closable @close="removeLabel(label)">
            {{ label }}
          </el-tag>
        </div>
        <div class="label-input">
          <el-input
            v-model="newLabel"
            placeholder="输入标签名称"
            size="small"
            @keyup.enter="addLabel"
          />
          <el-button size="small" type="primary" @click="addLabel">添加</el-button>
        </div>
      </div>
    </el-card>

    <el-card shadow="never" class="settings-card">
      <div class="section-title">通知与主题</div>
      <el-form :model="localSettings" label-width="90px" class="settings-form">
        <el-form-item label="新邮件通知">
          <el-switch v-model="localSettings.notify" />
        </el-form-item>
        <el-form-item label="主题">
          <el-select v-model="localSettings.theme" placeholder="请选择主题" style="width: 180px">
            <el-option v-for="item in themeOptions" :key="item" :label="item" :value="item" />
          </el-select>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<style scoped>
.settings-panel {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.settings-header {
  flex: 0 0 100%;
}

.settings-card {
  flex: 1 1 320px;
  min-width: 280px;
}

.settings-card--wide {
  flex: 2 1 520px;
}



.settings-header {
  background: #ffffff;
  border: 1px solid #e5ecf6;
  border-radius: 12px;
  padding: 16px 18px;
}

.settings-title {
  font-size: 18px;
  font-weight: 600;
  color: #2b3a55;
}

.settings-subtitle {
  margin-top: 4px;
  font-size: 12px;
  color: #8a98b2;
}

.settings-card {
  border-radius: 12px;
  border-color: #e5ecf6;
}

.settings-card :deep(.el-card__body) {
  padding: 16px 18px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  color: #2b3a55;
  margin-bottom: 12px;
}

.settings-form :deep(.el-form-item) {
  margin-bottom: 14px;
}

.signature-editor {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.signature-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.signature-input {
  min-height: 120px;
  border: 1px solid #e5ecf6;
  border-radius: 8px;
  padding: 10px 12px;
  font-size: 13px;
  line-height: 1.6;
  color: #2f3d55;
  background: #ffffff;
  outline: none;
}

.signature-input:empty::before {
  content: attr(data-placeholder);
  color: #b0b9c9;
}

.signature-input img {
  max-width: 140px;
  height: auto;
  border-radius: 6px;
  margin-top: 6px;
}

.signature-tip {
  font-size: 12px;
  color: #8a98b2;
}

.settings-tip {
  margin-top: 6px;
  font-size: 12px;
  color: #8a98b2;
}


.upload-btn {
  display: inline-flex;
  align-items: center;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid #d3dbe8;
  font-size: 12px;
  color: #5b6d88;
  cursor: pointer;
}

.upload-btn input {
  display: none;
}

.label-manager {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.label-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.label-input {
  display: flex;
  align-items: center;
  gap: 8px;
  max-width: 320px;
}
</style>
