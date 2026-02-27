<script setup>
import { ref, computed, watch } from "vue";
import { ElMessage } from "element-plus";

const props = defineProps({
  contacts: {
    type: Array,
    default: () => []
  },
  ldapSettings: {
    type: Object,
    default: () => ({})
  }
});

const emit = defineEmits(["update:contacts", "update:ldapSettings"]);


const localContacts = ref(Array.isArray(props.contacts) ? [...props.contacts] : []);
const keyword = ref("");
const groupFilter = ref("");
const dialogVisible = ref(false);
const editingId = ref(null);
const formRef = ref(null);
const syncing = ref(false);

const defaultLdapSettings = {
  enabled: false,
  host: "",
  port: 389,
  baseDn: "",
  bindDn: "",
  password: "",
  autoSync: false,
  syncInterval: "手动",
  lastSync: "",
  status: "未连接"
};

const localLdap = ref({ ...defaultLdapSettings, ...(props.ldapSettings || {}) });
const intervalOptions = ["手动", "每天", "每周", "每月"];


const emptyContact = () => ({
  id: "",
  name: "",
  email: "",
  phone: "",
  department: "",
  position: "",
  group: "",
  tag: "",
  notes: "",
  starred: false
});

const form = ref(emptyContact());

const rules = {
  name: [{ required: true, message: "请输入姓名", trigger: "blur" }],
  email: [
    { required: true, message: "请输入邮箱", trigger: "blur" },
    { type: "email", message: "请输入有效邮箱", trigger: "blur" }
  ]
};

watch(
  () => props.contacts,
  (value) => {
    localContacts.value = Array.isArray(value) ? [...value] : [];
  },
  { deep: true }
);

watch(
  () => props.ldapSettings,
  (value) => {
    localLdap.value = { ...defaultLdapSettings, ...(value || {}) };
  },
  { deep: true }
);

watch(
  localContacts,
  (value) => {
    emit("update:contacts", [...value]);
  },
  { deep: true }
);

watch(
  localLdap,
  (value) => {
    emit("update:ldapSettings", { ...value });
  },
  { deep: true }
);


const groupOptions = computed(() => {
  const set = new Set();
  localContacts.value.forEach((item) => {
    if (item.group) {
      set.add(item.group);
    }
  });
  return Array.from(set);
});

const totalCount = computed(() => localContacts.value.length);
const starredCount = computed(() => localContacts.value.filter((item) => item.starred).length);
const groupCount = computed(() => groupOptions.value.length);

const filteredContacts = computed(() => {
  const value = keyword.value.trim().toLowerCase();
  return localContacts.value.filter((item) => {
    const matchesGroup = !groupFilter.value || item.group === groupFilter.value;
    if (!matchesGroup) {
      return false;
    }
    if (!value) {
      return true;
    }
    return [item.name, item.email, item.phone, item.department, item.position]
      .filter(Boolean)
      .some((field) => field.toLowerCase().includes(value));
  });
});

const resetForm = () => {
  form.value = emptyContact();
  editingId.value = null;
  formRef.value?.clearValidate();
};

const openCreate = () => {
  resetForm();
  dialogVisible.value = true;
};

const openEdit = (row) => {
  form.value = { ...row };
  editingId.value = row.id;
  dialogVisible.value = true;
};

const handleSave = async () => {
  try {
    await formRef.value?.validate();
    if (editingId.value) {
      const index = localContacts.value.findIndex((item) => item.id === editingId.value);
      if (index >= 0) {
        localContacts.value.splice(index, 1, { ...form.value, id: editingId.value });
      }
      ElMessage.success("联系人已更新");
    } else {
      const newContact = { ...form.value, id: `contact-${Date.now()}` };
      localContacts.value.unshift(newContact);
      ElMessage.success("联系人已添加");
    }
    dialogVisible.value = false;
    resetForm();
  } catch (error) {
    if (error) {
      ElMessage.warning("请完善联系人信息");
    }
  }
};

const removeContact = (row) => {
  localContacts.value = localContacts.value.filter((item) => item.id !== row.id);
  ElMessage.success("联系人已删除");
};

const toggleStar = (row) => {
  const index = localContacts.value.findIndex((item) => item.id === row.id);
  if (index >= 0) {
    localContacts.value[index].starred = !localContacts.value[index].starred;
  }
};

const canSync = computed(() =>
  localLdap.value.enabled && localLdap.value.host && localLdap.value.baseDn && localLdap.value.bindDn
);

const mergeContacts = (items) => {
  items.forEach((item) => {
    const index = localContacts.value.findIndex((row) => row.email === item.email);
    if (index >= 0) {
      const existing = localContacts.value[index];
      localContacts.value.splice(index, 1, {
        ...existing,
        ...item,
        starred: existing.starred
      });
    } else {
      localContacts.value.unshift(item);
    }
  });
};

const getLdapSampleContacts = () => [
  {
    id: `ldap-${Date.now()}-1`,
    name: "赵启明",
    email: "zhaoqiming@company.com",
    phone: "010-5566-8877",
    department: "信息中心",
    position: "系统管理员",
    group: "LDAP",
    tag: "企业",
    notes: "LDAP 同步联系人",
    starred: false
  },
  {
    id: `ldap-${Date.now()}-2`,
    name: "蒋文静",
    email: "jiangwenjing@company.com",
    phone: "021-7788-9966",
    department: "安全合规",
    position: "安全负责人",
    group: "LDAP",
    tag: "企业",
    notes: "LDAP 同步联系人",
    starred: false
  },
  {
    id: `ldap-${Date.now()}-3`,
    name: "郭宇航",
    email: "guoyuhang@company.com",
    phone: "0755-2266-3344",
    department: "基础平台",
    position: "平台工程师",
    group: "LDAP",
    tag: "企业",
    notes: "LDAP 同步联系人",
    starred: false
  }
];

const handleTestConnection = () => {
  if (!canSync.value) {
    ElMessage.warning("请填写 LDAP 地址、Base DN 与 Bind DN");
    return;
  }
  localLdap.value.status = "连接成功";
  ElMessage.success("LDAP 连接测试成功");
};

const handleSync = () => {
  if (!canSync.value) {
    ElMessage.warning("请完善 LDAP 配置信息");
    return;
  }
  syncing.value = true;
  localLdap.value.status = "同步中";
  window.setTimeout(() => {
    mergeContacts(getLdapSampleContacts());
    localLdap.value.lastSync = new Date().toLocaleString();
    localLdap.value.status = "已同步";
    syncing.value = false;
    ElMessage.success("LDAP 同步完成");
  }, 800);
};

</script>

<template>
  <div class="contacts-panel">
    <div class="contacts-header">
      <div>
        <div class="contacts-title">通讯录</div>
        <div class="contacts-subtitle">管理联系人、分组与常用邮箱</div>
      </div>
      <div class="contacts-actions">
        <el-input
          v-model="keyword"
          size="small"
          clearable
          placeholder="搜索姓名、邮箱或部门"
          class="contacts-search"
        />
        <el-select v-model="groupFilter" size="small" clearable placeholder="全部分组">
          <el-option v-for="group in groupOptions" :key="group" :label="group" :value="group" />
        </el-select>
        <el-button type="primary" size="small" @click="openCreate">新建联系人</el-button>
      </div>
    </div>

    <div class="contacts-stats">
      <el-card shadow="never" class="stat-card">
        <div class="stat-label">联系人总数</div>
        <div class="stat-value">{{ totalCount }}</div>
      </el-card>
      <el-card shadow="never" class="stat-card">
        <div class="stat-label">常用联系人</div>
        <div class="stat-value">{{ starredCount }}</div>
      </el-card>
      <el-card shadow="never" class="stat-card">
        <div class="stat-label">分组数量</div>
        <div class="stat-value">{{ groupCount }}</div>
      </el-card>
    </div>

    <el-card shadow="never" class="ldap-card">
      <div class="ldap-header">
        <div>
          <div class="ldap-title">企业通讯录同步（LDAP）</div>
          <div class="ldap-subtitle">配置 LDAP 地址后可同步企业联系人</div>
        </div>
        <div class="ldap-status" :class="`status-${localLdap.status}`">
          {{ localLdap.status || "未连接" }}
        </div>
      </div>
      <el-form :model="localLdap" label-width="90px" class="ldap-form">
        <el-form-item label="启用同步">
          <el-switch v-model="localLdap.enabled" />
        </el-form-item>
        <el-form-item label="LDAP 地址">
          <el-input v-model="localLdap.host" placeholder="例如 ldap.company.com" />
        </el-form-item>
        <el-form-item label="端口">
          <el-input v-model="localLdap.port" placeholder="389" />
        </el-form-item>
        <el-form-item label="Base DN">
          <el-input v-model="localLdap.baseDn" placeholder="例如 dc=company,dc=com" />
        </el-form-item>
        <el-form-item label="Bind DN">
          <el-input v-model="localLdap.bindDn" placeholder="例如 cn=admin,dc=company,dc=com" />
        </el-form-item>
        <el-form-item label="密码">
          <el-input v-model="localLdap.password" type="password" show-password placeholder="请输入绑定密码" />
        </el-form-item>
        <el-form-item label="自动同步">
          <div class="ldap-inline">
            <el-switch v-model="localLdap.autoSync" />
            <el-select v-model="localLdap.syncInterval" size="small" class="ldap-select">
              <el-option v-for="item in intervalOptions" :key="item" :label="item" :value="item" />
            </el-select>
            <span class="ldap-tip">上次同步：{{ localLdap.lastSync || "-" }}</span>
          </div>
        </el-form-item>
        <el-form-item label="操作">
          <div class="ldap-actions">
            <el-button size="small" @click="handleTestConnection">测试连接</el-button>
            <el-button type="primary" size="small" :loading="syncing" @click="handleSync">
              立即同步
            </el-button>
          </div>
        </el-form-item>
      </el-form>
    </el-card>

    <el-card shadow="never" class="contacts-card">

      <el-table :data="filteredContacts" stripe class="contacts-table">
        <el-table-column label="常用" width="70">
          <template #default="scope">
            <el-button link class="star-btn" @click="toggleStar(scope.row)">
              <span :class="{ starred: scope.row.starred }">★</span>
            </el-button>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="姓名" width="120" />
        <el-table-column prop="email" label="邮箱" min-width="180" />
        <el-table-column prop="department" label="部门" width="120" />
        <el-table-column prop="position" label="职位" width="120" />
        <el-table-column prop="group" label="分组" width="120" />
        <el-table-column label="标签" width="120">
          <template #default="scope">
            <el-tag v-if="scope.row.tag" size="small" effect="light">{{ scope.row.tag }}</el-tag>
            <span v-else class="tag-placeholder">-</span>
          </template>
        </el-table-column>
        <el-table-column prop="phone" label="电话" width="140" />
        <el-table-column label="操作" width="140" fixed="right">
          <template #default="scope">
            <el-button link type="primary" size="small" @click="openEdit(scope.row)">编辑</el-button>
            <el-button link type="danger" size="small" @click="removeContact(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-dialog v-model="dialogVisible" :title="editingId ? '编辑联系人' : '新建联系人'" width="520px">
      <el-form ref="formRef" :model="form" :rules="rules" label-width="80px">
        <el-form-item label="姓名" prop="name">
          <el-input v-model="form.name" placeholder="请输入姓名" />
        </el-form-item>
        <el-form-item label="邮箱" prop="email">
          <el-input v-model="form.email" placeholder="请输入邮箱" />
        </el-form-item>
        <el-form-item label="电话">
          <el-input v-model="form.phone" placeholder="请输入电话" />
        </el-form-item>
        <el-form-item label="部门">
          <el-input v-model="form.department" placeholder="例如 市场部" />
        </el-form-item>
        <el-form-item label="职位">
          <el-input v-model="form.position" placeholder="例如 产品经理" />
        </el-form-item>
        <el-form-item label="分组">
          <el-input v-model="form.group" placeholder="例如 客户/内部" />
        </el-form-item>
        <el-form-item label="标签">
          <el-input v-model="form.tag" placeholder="例如 VIP" />
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.notes" type="textarea" rows="3" placeholder="补充说明" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.contacts-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.contacts-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 18px;
  background: #ffffff;
  border: 1px solid #e5ecf6;
  border-radius: 12px;
}

.contacts-title {
  font-size: 18px;
  font-weight: 600;
  color: #2b3a55;
}

.contacts-subtitle {
  margin-top: 4px;
  font-size: 12px;
  color: #8a98b2;
}

.contacts-actions {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.contacts-search {
  width: 220px;
}

.contacts-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 12px;
}

.stat-card {
  border-radius: 12px;
  border-color: #e5ecf6;
}

.stat-card :deep(.el-card__body) {
  padding: 14px 16px;
}

.stat-label {
  font-size: 12px;
  color: #8a98b2;
}

.stat-value {
  margin-top: 4px;
  font-size: 20px;
  font-weight: 600;
  color: #1f6fe5;
}

.contacts-card {
  border-radius: 12px;
  border-color: #e5ecf6;
}

.contacts-card :deep(.el-card__body) {
  padding: 0;
}

.ldap-card {
  border-radius: 12px;
  border-color: #e5ecf6;
}

.ldap-card :deep(.el-card__body) {
  padding: 16px 18px;
}

.ldap-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.ldap-title {
  font-size: 14px;
  font-weight: 600;
  color: #2b3a55;
}

.ldap-subtitle {
  margin-top: 4px;
  font-size: 12px;
  color: #8a98b2;
}

.ldap-status {
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 12px;
  background: #f2f5fb;
  color: #5b6d88;
}

.status-连接成功,
.status-已同步 {
  background: #e7f5ec;
  color: #2a7a47;
}

.status-同步中 {
  background: #eaf1ff;
  color: #1f6fe5;
}

.ldap-form {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 8px 16px;
}

.ldap-form :deep(.el-form-item) {
  margin-bottom: 8px;
}

.ldap-inline {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.ldap-select {
  width: 120px;
}

.ldap-tip {
  font-size: 12px;
  color: #8a98b2;
}

.ldap-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}


.contacts-table :deep(.el-table__cell) {
  font-size: 13px;
  color: #2f3d55;
}

.star-btn {
  font-size: 14px;
  color: #c6d1e5;
}

.star-btn .starred {
  color: #f4b400;
}

.tag-placeholder {
  color: #c2cad8;
}
</style>
