<script setup>
import { reactive, ref, watch, onMounted, nextTick } from "vue";

const formRef = ref(null);
const editorRef = ref(null);
const form = reactive({
  to: "",
  cc: "",
  subject: "",
  content: ""
});

const getPlainText = (html) => {
  const container = document.createElement("div");
  container.innerHTML = html || "";
  return (container.textContent || "").trim();
};

const contentValidator = (_rule, value, callback) => {
  if (!getPlainText(value)) {
    callback(new Error("请输入邮件正文"));
    return;
  }
  callback();
};

const rules = {
  to: [
    { required: true, message: "请输入收件人邮箱", trigger: "blur" },
    { type: "email", message: "请输入正确的邮箱格式", trigger: "blur" }
  ],
  cc: [{ type: "email", message: "请输入正确的邮箱格式", trigger: "blur" }],
  subject: [{ required: true, message: "请输入邮件主题", trigger: "blur" }],
  content: [{ validator: contentValidator, trigger: "blur" }]
};

const updateContent = () => {
  if (!editorRef.value) {
    return;
  }
  form.content = editorRef.value.innerHTML;
  formRef.value?.validateField("content");
};

const syncEditorContent = async () => {
  await nextTick();
  if (editorRef.value && editorRef.value.innerHTML !== form.content) {
    editorRef.value.innerHTML = form.content || "";
  }
};

watch(
  () => form.content,
  () => {
    syncEditorContent();
  }
);

onMounted(() => {
  if (editorRef.value) {
    editorRef.value.innerHTML = form.content || "";
  }
});

const validate = () => {
  return formRef.value?.validate();
};

const resetForm = () => {
  formRef.value?.resetFields();
  if (editorRef.value) {
    editorRef.value.innerHTML = "";
  }
};

defineExpose({
  form,
  validate,
  resetForm
});
</script>




<template>
  <el-form
    ref="formRef"
    class="compose-form"
    label-width="60px"
    :model="form"
    :rules="rules"
  >

    <el-form-item label="收件人" prop="to">

      <el-input v-model="form.to" placeholder="请输入收件人邮箱" />
    </el-form-item>
    <el-form-item label="抄送" prop="cc">

      <el-input v-model="form.cc" placeholder="请输入抄送邮箱" />
    </el-form-item>
    <el-form-item label="主题" prop="subject">

      <el-input v-model="form.subject" placeholder="请输入邮件主题" />
    </el-form-item>
    <el-form-item label="正文" prop="content">
      <div
        ref="editorRef"
        class="compose-editor"
        contenteditable="true"
        data-placeholder="请输入邮件内容..."
        @input="updateContent"
        @blur="updateContent"
      ></div>
      <div class="compose-helper">支持富文本内容与签名图片</div>
    </el-form-item>


  </el-form>
</template>

<style scoped>
.compose-form :deep(.el-form-item) {
  margin-bottom: 16px;
}

.compose-helper {
  margin-top: 6px;
  font-size: 12px;
  color: #8a98b2;
}


.compose-form :deep(.el-textarea__inner) {
  resize: none;
}

.compose-editor {
  min-height: 220px;
  border: 1px solid #e5ecf6;
  border-radius: 8px;
  padding: 10px 12px;
  background: #ffffff;
  font-size: 13px;
  line-height: 1.6;
  color: #2f3d55;
  outline: none;
}

.compose-editor:empty::before {
  content: attr(data-placeholder);
  color: #b0b9c9;
}

.compose-editor blockquote {
  margin: 12px 0;
  padding: 8px 12px;
  border-left: 3px solid #c9d7f2;
  background: #f6f8ff;
  color: #6c7a94;
}

.compose-editor img {
  max-width: 100%;
  height: auto;
  border-radius: 6px;
  margin: 6px 0;
}

</style>
