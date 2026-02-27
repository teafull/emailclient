<script setup>
import { reactive, ref } from "vue";

const formRef = ref(null);
const form = reactive({
  to: "",
  cc: "",
  subject: "",
  content: ""
});

const rules = {
  to: [
    { required: true, message: "请输入收件人邮箱", trigger: "blur" },
    { type: "email", message: "请输入正确的邮箱格式", trigger: "blur" }
  ],
  cc: [{ type: "email", message: "请输入正确的邮箱格式", trigger: "blur" }],
  subject: [{ required: true, message: "请输入邮件主题", trigger: "blur" }],
  content: [{ required: true, message: "请输入邮件正文", trigger: "blur" }]
};

const validate = () => {
  return formRef.value?.validate();
};

const resetForm = () => {
  formRef.value?.resetFields();
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

      <el-input
        v-model="form.content"
        type="textarea"
        :rows="10"
        placeholder="请输入邮件内容..."
      />
    </el-form-item>
  </el-form>
</template>

<style scoped>
.compose-form :deep(.el-form-item) {
  margin-bottom: 16px;
}

.compose-form :deep(.el-textarea__inner) {
  resize: none;
}
</style>
