<script setup>
import { ref, computed, onMounted } from "vue";
import { ElMessage } from "element-plus";

const emit = defineEmits(["login-success"]);

const formRef = ref(null);
const loading = ref(false);
const form = ref({
  email: "",
  password: ""
});
const rememberPassword = ref(false);

const rules = {
  email: [
    { required: true, message: "请输入邮箱地址", trigger: "blur" },
    { type: "email", message: "请输入正确的邮箱格式", trigger: "blur" }
  ],
  password: [
    { required: true, message: "请输入密码", trigger: "blur" },
    { min: 1, message: "密码不能为空", trigger: "blur" }
  ]
};

const readStorage = (key, fallback) => {
  const raw = localStorage.getItem(key);
  if (!raw) return fallback;
  try {
    return JSON.parse(raw);
  } catch {
    return fallback;
  }
};

onMounted(() => {
  const saved = readStorage("mailCredentials", null);
  if (saved) {
    form.value.email = saved.email || "";
    form.value.password = saved.password || "";
    rememberPassword.value = saved.remember || false;
  }
});

const handleLogin = async () => {
  try {
    await formRef.value?.validate();
  } catch {
    ElMessage.warning("请完善登录信息");
    return;
  }

  loading.value = true;

  // Simulate login verification
  setTimeout(() => {
    if (form.value.email && form.value.password) {
      if (rememberPassword.value) {
        localStorage.setItem(
          "mailCredentials",
          JSON.stringify({
            email: form.value.email,
            password: form.value.password,
            remember: true
          })
        );
      } else {
        localStorage.removeItem("mailCredentials");
      }

      localStorage.setItem(
        "mailSettings",
        JSON.stringify({
          ...JSON.parse(localStorage.getItem("mailSettings") || "{}"),
          email: form.value.email,
          emailPassword: form.value.password
        })
      );

      loading.value = false;
      ElMessage.success("登录成功");
      emit("login-success");
    } else {
      loading.value = false;
      ElMessage.error("邮箱或密码错误");
    }
  }, 800);
};
</script>

<template>
  <div class="login-container">
    <div class="login-card">
      <div class="login-header">
        <div class="login-logo">
          <svg viewBox="0 0 24 24" width="48" height="48" fill="none">
            <rect x="2" y="4" width="20" height="16" rx="2" stroke="#1f6fe5" stroke-width="2"/>
            <path d="M2 8l10 6 10-6" stroke="#1f6fe5" stroke-width="2"/>
          </svg>
        </div>
        <h1 class="login-title">邮箱客户端</h1>
        <p class="login-subtitle">请登录您的邮箱账号</p>
      </div>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        class="login-form"
        @submit.prevent="handleLogin"
      >
        <el-form-item prop="email">
          <el-input
            v-model="form.email"
            placeholder="请输入邮箱地址"
            size="large"
            prefix-icon="Message"
            clearable
          />
        </el-form-item>

        <el-form-item prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            size="large"
            prefix-icon="Lock"
            show-password
            @keyup.enter="handleLogin"
          />
        </el-form-item>

        <div class="login-options">
          <el-checkbox v-model="rememberPassword">记住密码</el-checkbox>
        </div>

        <el-form-item>
          <el-button
            type="primary"
            size="large"
            :loading="loading"
            class="login-btn"
            @click="handleLogin"
          >
            登录
          </el-button>
        </el-form-item>
      </el-form>

      <div class="login-footer">
        <span class="login-hint">测试账号：test@example.com / password</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #1d63d8 0%, #1f73ea 50%, #3b82f6 100%);
}

.login-card {
  width: 400px;
  padding: 40px 36px;
  background: #ffffff;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-logo {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 72px;
  height: 72px;
  background: #e9f1ff;
  border-radius: 16px;
  margin-bottom: 16px;
}

.login-title {
  font-size: 24px;
  font-weight: 600;
  color: #1f2f44;
  margin: 0 0 8px;
}

.login-subtitle {
  font-size: 14px;
  color: #8a98b2;
  margin: 0;
}

.login-form {
  margin-bottom: 24px;
}

.login-options {
  display: flex;
  align-items: center;
  margin-bottom: 24px;
}

.login-btn {
  width: 100%;
  height: 44px;
  font-size: 16px;
  border-radius: 10px;
}

.login-footer {
  text-align: center;
}

.login-hint {
  font-size: 12px;
  color: #8a98b2;
}
</style>
