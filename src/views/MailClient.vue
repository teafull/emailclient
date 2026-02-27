<script setup>
import { ref, nextTick, computed, watch } from "vue";

import { ElMessage } from "element-plus";
import ComposeMail from "./ComposeMail.vue";
import ContactsView from "./ContactsView.vue";
import SettingsView from "./SettingsView.vue";


const composeVisible = ref(false);
const composeRef = ref(null);
const detailVisible = ref(false);
const selectedMail = ref(null);
const currentView = ref("mail");
const activeMenu = ref("收件箱");

const handleMenuSelect = (index) => {
  activeMenu.value = index;
  if (index === "设置") {
    currentView.value = "settings";
    return;
  }
  if (index === "通讯录") {
    currentView.value = "contacts";
    return;
  }
  currentView.value = "mail";
};


const openMailDetail = (mail) => {
  selectedMail.value = mail;
  detailVisible.value = true;
};

const getSignatureBlock = (leadingBlank = true) => {
  const signatureHtml = getSignatureHtml();
  if (!signatureHtml) {
    return "";
  }
  return `${leadingBlank ? "<br /><br />" : ""}<div class="mail-signature">--<br />${signatureHtml}</div>`;
};


const prefixSubject = (prefix, subject) => {

  if (!subject) {
    return prefix.trim();
  }
  return subject.startsWith(prefix) ? subject : `${prefix}${subject}`;
};

const formatCc = (cc) => {
  if (!cc) {
    return [];
  }
  return Array.isArray(cc) ? cc : [cc];
};

const openComposeWithAction = async (action) => {
  const mail = selectedMail.value;
  if (!mail) {
    return;
  }
  composeVisible.value = true;
  await nextTick();
  const form = composeRef.value?.form;
  if (!form) {
    return;
  }
  form.to = "";
  form.cc = "";
  form.subject = "";
  form.content = "";

  const signature = getSignatureBlock(true);
  const replyQuoteBody = textToHtml(mail.content || mail.preview);
  const replyQuote = `<p>在 ${mail.time}，${mail.sender} 写道：</p><blockquote>${replyQuoteBody}</blockquote>`;
  const forwardQuote = `<p>--- 转发邮件 ---</p><p>发件人：${mail.sender}<br />时间：${mail.time}<br />主题：${mail.subject}</p><p>${textToHtml(mail.content || mail.preview)}</p>`;

  if (action === "reply") {
    form.to = mail.email || "";
    form.subject = prefixSubject("Re: ", mail.subject);
    form.content = `${replyQuote}${signature}`;
    return;
  }

  if (action === "replyAll") {
    form.to = mail.email || "";
    form.cc = formatCc(mail.cc).join(", ");
    form.subject = prefixSubject("Re: ", mail.subject);
    form.content = `${replyQuote}${signature}`;
    return;
  }

  if (action === "forward") {
    form.subject = prefixSubject("Fwd: ", mail.subject);
    form.content = `${forwardQuote}${signature}`;
  }
};






const addTodoFromMail = (mail) => {
  if (!mail) {
    return;
  }
  const newTodo = {
    title: `跟进：${mail.subject}`,
    source: mail.sender,
    time: new Date().toLocaleString(),
    status: "待处理"
  };
  todos.value.unshift(newTodo);
  ElMessage.success("已加入待办");
};

const extractQuoteHtml = (content) => {
  if (!content) {
    return "";
  }
  const forwardIndex = content.indexOf("--- 转发邮件 ---");
  if (forwardIndex >= 0) {
    return content.slice(forwardIndex - 3);
  }
  const match = content.match(/<blockquote[\s\S]*<\/blockquote>/);
  return match ? match[0] : "";
};

const generateMailContent = () => {
  const form = composeRef.value?.form;
  if (!form) {
    return;
  }
  const llm = { ...defaultSettings.llm, ...(settings.value.llm || {}) };
  if (!llm.enabled) {
    ElMessage.warning("大模型已关闭");
    return;
  }
  if (!llm.model) {
    ElMessage.warning("请在设置中填写模型名称");
    return;
  }
  const tone = llm.tone || "专业";
  const subject = form.subject || "邮件主题";
  const receiver = form.to || "收件人";
  const signature = getSignatureBlock(true);
  const quoteHtml = extractQuoteHtml(form.content);
  const body = `<p>${receiver} 您好：</p><p>关于“${subject}”，我已基于${tone}风格生成一版内容草稿，请您查看并调整细节。</p><p>如有需要补充的信息，请告诉我，我会继续完善。</p>`;
  form.content = `${body}${quoteHtml}${signature}`;
  ElMessage.success("已生成邮件草稿");
};

const openComposeNew = async () => {
  composeVisible.value = true;
  await nextTick();
  const form = composeRef.value?.form;
  if (!form) {
    return;
  }
  form.to = "";
  form.cc = "";
  form.subject = "";
  const signature = getSignatureBlock(false);
  form.content = signature ? signature : "";
};

const handleSend = async () => {


  try {
    await composeRef.value?.validate();
    ElMessage.success("邮件已发送");
    composeRef.value?.resetForm();
    composeVisible.value = false;
  } catch (error) {
    if (error) {
      ElMessage.warning("请完善必填项后再发送");
    }
  }
};

const saveDraft = () => {
  const form = composeRef.value?.form;
  if (!form) {
    return;
  }
  const hasContent = form.to || form.cc || form.subject || form.content;
  if (!hasContent) {
    ElMessage.warning("草稿内容为空");
    return;
  }
  const draft = {
    to: form.to,
    cc: form.cc,
    subject: form.subject,
    content: form.content,
    time: new Date().toLocaleString()
  };
  let drafts = [];
  try {
    drafts = JSON.parse(localStorage.getItem("mailDrafts") || "[]");
  } catch (error) {
    drafts = [];
  }
  drafts.unshift(draft);
  localStorage.setItem("mailDrafts", JSON.stringify(drafts));
  ElMessage.success("草稿已保存");
};

const readStorage = (key, fallback) => {
  const raw = localStorage.getItem(key);
  if (!raw) {
    return fallback;
  }
  try {
    return JSON.parse(raw);
  } catch (error) {
    return fallback;
  }
};

const escapeHtml = (value) =>
  (value || "")
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");

const textToHtml = (value) => escapeHtml(value).replace(/\n/g, "<br />");

const defaultSettings = {
  displayName: "张三",
  title: "市场与运营部",
  email: "zhangsan@company.com",
  emailPassword: "",
  signature: "张三\n市场与运营部\n电话：010-12345678",

  signatureHtml: "张三<br />市场与运营部<br />电话：010-12345678",
  notify: true,
  autoDraft: true,
  theme: "专业蓝",
  llm: {
    enabled: true,
    provider: "OpenAI",
    model: "gpt-4o-mini",
    apiKey: "",
    endpoint: "",
    tone: "专业"
  }
};

const storedSettings = readStorage("mailSettings", {});

const settings = ref({
  ...defaultSettings,
  ...storedSettings,
  llm: { ...defaultSettings.llm, ...(storedSettings.llm || {}) }
});


const getSignatureHtml = () => {
  if (settings.value.signatureHtml) {
    return settings.value.signatureHtml;
  }
  if (settings.value.signature) {
    return textToHtml(settings.value.signature);
  }
  return "";
};


const labels = ref(readStorage("mailLabels", ["重要", "工作", "个人"]).filter(Boolean));

const contacts = ref(
  readStorage("mailContacts", [
    {
      id: "contact-001",
      name: "李婷",
      email: "liting@company.com",
      phone: "138-0000-2345",
      department: "市场部",
      position: "市场经理",
      group: "客户",
      tag: "VIP",
      notes: "重点客户，偏好月度总结",
      starred: true
    },
    {
      id: "contact-002",
      name: "周铭",
      email: "zhouming@company.com",
      phone: "139-1122-5566",
      department: "产品部",
      position: "产品负责人",
      group: "内部",
      tag: "项目",
      notes: "对接 CRM 项目",
      starred: false
    },
    {
      id: "contact-003",
      name: "王珊",
      email: "wangshan@company.com",
      phone: "137-8899-7788",
      department: "人力资源",
      position: "HRBP",
      group: "内部",
      tag: "招聘",
      notes: "校招项目跟进",
      starred: false
    },
    {
      id: "contact-004",
      name: "陈宇",
      email: "chenyu@company.com",
      phone: "136-5566-8899",
      department: "客户成功",
      position: "客户成功经理",
      group: "客户",
      tag: "续约",
      notes: "合同将于下月到期",
      starred: true
    }
  ])
);

const ldapSettings = ref(
  readStorage("mailContactsLdap", {
    enabled: false,
    host: "ldap.company.com",
    port: 389,
    baseDn: "dc=company,dc=com",
    bindDn: "cn=admin,dc=company,dc=com",
    password: "",
    autoSync: false,
    syncInterval: "手动",
    lastSync: "",
    status: "未连接"
  })
);



const labelToneMap = {
  重要: "danger",
  工作: "work",
  个人: "personal"
};

const labelsWithTone = computed(() =>
  labels.value.map((name) => ({
    name,
    tone: labelToneMap[name] || "default"
  }))
);

watch(
  settings,
  (value) => {
    localStorage.setItem("mailSettings", JSON.stringify(value));
  },
  { deep: true }
);

watch(
  labels,
  (value) => {
    localStorage.setItem("mailLabels", JSON.stringify(value));
  },
  { deep: true }
);

watch(
  contacts,
  (value) => {
    localStorage.setItem("mailContacts", JSON.stringify(value));
  },
  { deep: true }
);

watch(
  ldapSettings,
  (value) => {
    localStorage.setItem("mailContactsLdap", JSON.stringify(value));
  },
  { deep: true }
);



const folders = [
  { name: "收件箱", count: 42, icon: "inbox", active: true },
  { name: "星标", count: 12, icon: "star" },
  { name: "已发送", count: 128, icon: "send" },
  { name: "草稿", count: 5, icon: "draft" },
  { name: "垃圾邮件", count: 26, icon: "trash" }
];

const systemMenus = [
  { name: "通讯录", icon: "contacts" },
  { name: "设置", icon: "settings" }
];




const tools = [
  { name: "刷新", icon: "refresh" },
  { name: "删除", icon: "delete" },
  { name: "移动", icon: "move" },
  { name: "标记为已读", icon: "read" }
];

const mails = [
  {
    sender: "技术支持团队",
    email: "support@company.com",
    cc: ["ops@company.com", "it@company.com", "sec@company.com", "ops2@company.com"],

    subject: "系统维护通知",
    preview: "本周六凌晨2:00-4:00将进行系统维护...",
    content:
      "您好，为确保系统稳定运行，本周六凌晨2:00-4:00将进行系统维护。期间可能出现短暂不可用，敬请谅解。如有紧急问题请联系技术支持。",
    tag: "重要",
    time: "今天 09:30",
    starred: true,
    active: true
  },
  {
    sender: "设计部门",
    email: "design@company.com",
    cc: ["pm@company.com", "ux@company.com"],

    subject: "新版本UI设计稿反馈",
    preview: "请查看附件中的设计稿并提供反馈意见...",
    content: "新版本UI设计稿已更新，请重点关注导航与列表样式。期待今天下午前给出反馈建议。",
    tag: "工作",
    time: "今天 08:45",
    starred: false,
    active: false
  },
  {
    sender: "人力资源部",
    email: "hr@company.com",
    subject: "团队建设活动邀请",
    preview: "本周五下午将举办季度团队建设活动...",
    content: "本周五下午将举办季度团队建设活动，请在今天18:00前完成报名并填写饮食偏好。",
    tag: "",
    time: "昨天 16:20",
    starred: false,
    active: false
  },
  {
    sender: "财务部门",
    email: "finance@company.com",
    subject: "工资单已发布",
    preview: "本月工资单已生成，请及时查看...",
    content: "本月工资单已发布，请在系统中查看并确认。如有疑问请联系财务。",
    tag: "",
    time: "昨天 14:15",
    starred: false,
    active: false
  }
];

const todos = ref([
  {
    title: "回复系统维护通知",
    source: "技术支持团队",
    time: "今天 10:30",
    status: "进行中"
  },
  {
    title: "跟进设计稿反馈",
    source: "设计部门",
    time: "今天 14:00",
    status: "待处理"
  },
  {
    title: "确认团队建设报名",
    source: "人力资源部",
    time: "明天 09:00",
    status: "待处理"
  }
]);
</script>

<template>
  <div class="mail-app">
    <el-container class="layout">
      <el-header class="topbar">
        <div class="brand">邮箱客户端</div>
        <div class="topbar-actions">
          <el-button class="compose-btn" type="primary" @click="openComposeNew">
            写邮件
          </el-button>

          <el-input
            class="search-input"
            size="small"
            placeholder="搜索邮件..."
            clearable
          />
        </div>
      </el-header>

      <el-container class="body">
        <el-aside width="200px" class="sidebar">
          <div class="section-title">邮箱</div>
          <el-menu class="nav" :default-active="activeMenu" @select="handleMenuSelect">

            <el-menu-item
              v-for="folder in folders"
              :key="folder.name"
              :index="folder.name"
              :class="{ active: folder.active }"
            >
              <span class="nav-icon" :class="`icon-${folder.icon}`"></span>
              <span class="nav-text">{{ folder.name }}</span>
              <el-badge class="nav-badge" :value="folder.count" :max="999" />
            </el-menu-item>
          </el-menu>

          <div class="section-title labels">标签</div>
          <div class="label-list">
            <div v-for="label in labelsWithTone" :key="label.name" class="label-item">
              <span class="label-dot" :class="`tone-${label.tone}`"></span>
              <span>{{ label.name }}</span>
            </div>
          </div>


          <div class="section-title system">系统</div>
          <el-menu class="nav system-nav" :default-active="activeMenu" @select="handleMenuSelect">
            <el-menu-item
              v-for="menu in systemMenus"
              :key="menu.name"
              :index="menu.name"
            >
              <span class="nav-icon" :class="`icon-${menu.icon}`"></span>
              <span class="nav-text">{{ menu.name }}</span>
            </el-menu-item>
          </el-menu>
        </el-aside>


        <el-main class="main">
          <div v-if="currentView === 'mail'">
            <div class="toolbar">
              <el-checkbox class="tool-check">全选</el-checkbox>
              <el-button v-for="tool in tools" :key="tool.name" link class="tool-btn">
                <span class="tool-icon" :class="`tool-${tool.icon}`"></span>
                {{ tool.name }}
              </el-button>
            </div>

            <div class="mail-list">
              <div
                v-for="mail in mails"
                :key="mail.subject"
                class="mail-row"
                :class="{ active: mail.active }"
                @dblclick="openMailDetail(mail)"
              >
                <el-checkbox class="row-check" />
                <span class="star" :class="{ starred: mail.starred }">★</span>
                <span class="sender">{{ mail.sender }}</span>
                <div class="subject">
                  <span class="subject-title">{{ mail.subject }}</span>
                  <span class="subject-preview"> - {{ mail.preview }}</span>
                </div>
                <el-tag
                  v-if="mail.tag"
                  class="tag"
                  :class="`tag-${mail.tag}`"
                  size="small"
                  effect="light"
                >
                  {{ mail.tag }}
                </el-tag>
                <span v-else class="tag tag-empty">&nbsp;</span>
                <el-button class="todo-action" link @click.stop="addTodoFromMail(mail)">
                  设为待办
                </el-button>
                <span class="time">{{ mail.time }}</span>
              </div>
            </div>
          </div>

          <ContactsView
            v-else-if="currentView === 'contacts'"
            class="contacts-view"
            v-model:contacts="contacts"
            v-model:ldapSettings="ldapSettings"
          />


          <SettingsView
            v-else
            class="settings-view"
            v-model:labels="labels"
            v-model:settings="settings"
          />


        </el-main>

        <el-aside v-if="currentView === 'mail'" class="todo-panel">

          <div class="todo-header">
            <span class="todo-title">待办列表</span>
            <el-tag size="small" effect="light" type="info">{{ todos.length }}</el-tag>
          </div>
          <div class="todo-list">
            <el-card
              v-for="todo in todos"
              :key="todo.title"
              class="todo-card"
              shadow="never"
            >
              <div class="todo-card-title">{{ todo.title }}</div>
              <div class="todo-card-meta">
                <span>{{ todo.source }}</span>
                <span>{{ todo.time }}</span>
              </div>
              <el-tag size="small" type="primary" effect="light">
                {{ todo.status }}
              </el-tag>
            </el-card>
          </div>
        </el-aside>
      </el-container>

      <el-dialog v-model="composeVisible" title="写邮件" width="720px">
        <ComposeMail ref="composeRef" />
        <template #footer>
          <el-button @click="composeVisible = false">取消</el-button>
          <el-button @click="generateMailContent">AI生成</el-button>
          <el-button @click="saveDraft">保存草稿</el-button>
          <el-button type="primary" @click="handleSend">发送</el-button>
        </template>
      </el-dialog>


      <el-dialog v-model="detailVisible" title="邮件详情" width="620px">
        <div v-if="selectedMail" class="detail-body">
          <div class="detail-actions">
            <el-button size="small" @click="openComposeWithAction('reply')">回复</el-button>
            <el-button size="small" @click="openComposeWithAction('replyAll')">
              全部回复
            </el-button>
            <el-button size="small" @click="openComposeWithAction('forward')">转发</el-button>
            <el-button size="small" @click="addTodoFromMail(selectedMail)">
              设为待办
            </el-button>
          </div>
          <div class="detail-title">{{ selectedMail.subject }}</div>
          <div class="detail-meta">
            <span>发件人：{{ selectedMail.sender }}</span>
            <span>时间：{{ selectedMail.time }}</span>
          </div>
          <div class="detail-cc" v-if="formatCc(selectedMail.cc).length">
            <span class="detail-cc-label">抄送：</span>
            <el-tooltip
              v-if="formatCc(selectedMail.cc).length > 3"
              placement="top"
              :content="formatCc(selectedMail.cc).join('、')"
            >
              <div class="detail-cc-tags">
                <el-tag v-for="cc in formatCc(selectedMail.cc).slice(0, 3)" :key="cc" size="small">
                  {{ cc }}
                </el-tag>
                <el-tag size="small" type="info">+{{ formatCc(selectedMail.cc).length - 3 }}</el-tag>
              </div>
            </el-tooltip>
            <div v-else class="detail-cc-tags">
              <el-tag v-for="cc in formatCc(selectedMail.cc)" :key="cc" size="small">
                {{ cc }}
              </el-tag>
            </div>
          </div>

          <div class="detail-content">
            {{ selectedMail.content || selectedMail.preview }}
          </div>
        </div>
        <template #footer>
          <el-button type="primary" @click="detailVisible = false">关闭</el-button>
        </template>
      </el-dialog>
    </el-container>
  </div>
</template>

<style scoped>
.mail-app {
  min-height: 100vh;
  background: #f2f5fb;
  --el-color-primary: #1f6fe5;
  --el-color-primary-light-3: #6ea3ff;
  --el-color-primary-light-7: #dbe8ff;
}

.layout {
  min-height: 100vh;
}

.topbar {
  height: 64px;
  background: linear-gradient(90deg, #1d63d8 0%, #1f73ea 100%);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  box-shadow: 0 6px 16px rgba(25, 74, 150, 0.24);
}

.brand {
  font-size: 20px;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.topbar-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.compose-btn {
  border-radius: 16px;
  padding: 0 16px;
  height: 32px;
  font-size: 13px;
  font-weight: 600;
  box-shadow: 0 6px 14px rgba(25, 88, 180, 0.24);
}

.search-input {
  width: 220px;
}

.search-input :deep(.el-input__wrapper) {
  border-radius: 18px;
  background: rgba(255, 255, 255, 0.2);
  box-shadow: none;
  border: 1px solid rgba(255, 255, 255, 0.3);
}

.search-input :deep(.el-input__inner) {
  color: #fff;
  font-size: 13px;
}

.search-input :deep(.el-input__inner::placeholder) {
  color: rgba(255, 255, 255, 0.7);
}

.body {
  flex: 1;
  background: #f2f5fb;
}

.sidebar {
  background: #fff;
  border-right: 1px solid #e5ecf6;
  padding: 18px 12px 16px;
}

.section-title {
  color: #91a1bc;
  font-size: 12px;
  font-weight: 600;
  margin: 6px 10px;
  letter-spacing: 1px;
}

.section-title.labels {
  margin-top: 12px;
}

.section-title.system {
  margin-top: 16px;
}


.nav {
  border-right: none;
  background: transparent;
}

.nav :deep(.el-menu-item) {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 9px 12px;
  border-radius: 10px;
  color: #2d3b55;
  font-size: 14px;
  height: auto;
  line-height: 1.4;
  margin: 2px 4px;
}

.nav :deep(.el-menu-item.is-active) {
  background: #e9f1ff;
  color: #1f6fe5;
  font-weight: 600;
}

.nav :deep(.el-menu-item:hover) {
  background: #f3f7ff;
}

.nav-icon {
  width: 16px;
  height: 16px;
  border-radius: 4px;
  background: #c8d6ee;
  position: relative;
}

.el-menu-item.is-active .nav-icon {
  background: #1f6fe5;
}

.nav-text {
  flex: 1;
}

.nav-icon.icon-star::after {
  content: "★";
  color: #fff;
  font-size: 10px;
  position: absolute;
  top: 1px;
  left: 3px;
}

.nav-icon.icon-inbox::after {
  content: "";
  position: absolute;
  inset: 4px 3px 3px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-top: 0;
  border-radius: 2px;
}

.nav-icon.icon-send::after {
  content: "";
  position: absolute;
  width: 0;
  height: 0;
  border-top: 4px solid transparent;
  border-bottom: 4px solid transparent;
  border-left: 7px solid rgba(255, 255, 255, 0.85);
  top: 4px;
  left: 5px;
}

.nav-icon.icon-draft::after {
  content: "";
  position: absolute;
  inset: 4px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-radius: 2px;
}

.nav-icon.icon-trash::after {
  content: "";
  position: absolute;
  width: 8px;
  height: 8px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-top: 0;
  border-radius: 0 0 2px 2px;
  top: 5px;
  left: 4px;
}

.nav-icon.icon-contacts::after {
  content: "";
  position: absolute;
  width: 8px;
  height: 8px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-radius: 50%;
  top: 3px;
  left: 3px;
}

.nav-icon.icon-contacts::before {
  content: "";
  position: absolute;
  width: 10px;
  height: 5px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-top: 0;
  border-radius: 0 0 8px 8px;
  bottom: 2px;
  left: 2px;
}

.nav-icon.icon-settings::after {
  content: "";
  position: absolute;
  inset: 4px;
  border: 2px solid rgba(255, 255, 255, 0.85);
  border-radius: 50%;
}



.nav-badge {
  margin-left: auto;
}

.nav-badge :deep(.el-badge__content) {
  background: #eef2f8;
  color: #5b6d88;
  border: none;
  height: 18px;
  min-width: 18px;
  padding: 0 6px;
  line-height: 18px;
  transform: translate(0, 0);
}

.el-menu-item.is-active .nav-badge :deep(.el-badge__content) {
  background: #1f6fe5;
  color: #fff;
}

.label-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 4px;
  padding-left: 6px;
}

.label-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  font-size: 14px;
  color: #2d3b55;
}

.label-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #d36b6b;
}

.label-dot.tone-danger {
  background: #f05b5b;
}

.label-dot.tone-work {
  background: #8b6fe7;
}

.label-dot.tone-personal {
  background: #38b36b;
}

.label-dot.tone-default {
  background: #7a8aa6;
}


.main {
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.toolbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 10px 14px;
  background: #fff;
  border: 1px solid #e5ecf6;
  border-radius: 10px;
}

.tool-check :deep(.el-checkbox__label) {
  color: #6b7b94;
  font-size: 13px;
}

.tool-btn {
  color: #6b7b94;
  font-size: 13px;
  padding: 0 4px;
}

.tool-icon {
  width: 14px;
  height: 14px;
  border-radius: 3px;
  border: 1.5px solid #b3c0d8;
  position: relative;
  margin-right: 6px;
  display: inline-block;
}

.tool-icon.tool-refresh::after {
  content: "";
  position: absolute;
  inset: 2px;
  border: 2px solid #b3c0d8;
  border-right-color: transparent;
  border-radius: 50%;
}

.tool-icon.tool-delete::after {
  content: "";
  position: absolute;
  width: 8px;
  height: 8px;
  border: 2px solid #b3c0d8;
  border-top: 0;
  border-radius: 0 0 2px 2px;
  top: 3px;
  left: 3px;
}

.tool-icon.tool-move::after {
  content: "";
  position: absolute;
  width: 6px;
  height: 6px;
  border: 2px solid #b3c0d8;
  border-right: 0;
  border-bottom: 0;
  transform: rotate(45deg);
  top: 3px;
  left: 4px;
}

.tool-icon.tool-read::after {
  content: "";
  position: absolute;
  width: 6px;
  height: 4px;
  border: 2px solid #b3c0d8;
  border-top: 0;
  top: 4px;
  left: 3px;
}

.mail-list {
  flex: 1;
  background: #fff;
  border: 1px solid #e5ecf6;
  border-radius: 10px;
  overflow: hidden;
}

.mail-row {
  display: grid;
  grid-template-columns: 26px 24px 160px 1fr 80px 90px 90px;
  gap: 12px;
  align-items: center;
  padding: 12px 16px;
  font-size: 13px;
  color: #2f3d55;
  border-top: 1px solid #eef2f7;
}

.mail-row:first-child {
  border-top: none;
}

.mail-row.active {
  background: #edf3ff;
}

.mail-row:hover {
  background: #f5f8ff;
  cursor: pointer;
}

.row-check :deep(.el-checkbox__inner) {
  width: 14px;
  height: 14px;
  border-radius: 3px;
  border: 1.5px solid #b7c5dc;
}

.star {
  font-size: 14px;
  color: #c6d1e5;
}

.star.starred {
  color: #f4b400;
}

.sender {
  font-weight: 600;
  color: #2b3a55;
}

.subject {
  color: #5a6a84;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.subject-title {
  color: #2f3d55;
  font-weight: 600;
}

.subject-preview {
  color: #7b8aa6;
}

.tag {
  justify-self: end;
  font-size: 12px;
}

.tag-重要 :deep(.el-tag) {
  background: #ffe9e9;
  color: #e05757;
  border-color: transparent;
}

.tag-工作 :deep(.el-tag) {
  background: #efe8ff;
  color: #7c5ce2;
  border-color: transparent;
}

.tag-empty {
  background: transparent;
}

.todo-action {
  justify-self: end;
  font-size: 12px;
  color: #1f6fe5;
}

.todo-action:hover {
  color: #1b61cc;
}

.time {
  justify-self: end;
  color: #8696b0;
  font-size: 12px;
}

.detail-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
  color: #2f3d55;
}

.detail-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.detail-title {
  font-size: 16px;
  font-weight: 600;
}

.detail-meta {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #8a98b2;
}

.detail-content {
  font-size: 13px;
  line-height: 1.8;
  color: #4c5b75;
  white-space: pre-wrap;
}

.detail-cc {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
  font-size: 12px;
  color: #6f7f98;
}

.detail-cc-label {
  font-weight: 600;
}

.detail-cc-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}


.mail-signature {
  color: #7a8aa6;
}

.mail-signature img {
  max-width: 140px;
  height: auto;
  display: inline-block;
}


.todo-panel {
  width: 260px;
  background: #fff;
  border: 1px solid #e5ecf6;
  border-radius: 10px;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin: 16px 16px 16px 0;
  height: fit-content;
}

.todo-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.todo-title {
  font-size: 14px;
  font-weight: 600;
  color: #2d3b55;
}

.todo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.todo-card {
  border-radius: 10px;
  border-color: #eef2f7;
}

.todo-card :deep(.el-card__body) {
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.todo-card-title {
  font-size: 13px;
  font-weight: 600;
  color: #2f3d55;
}

.todo-card-meta {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #8a98b2;
}
</style>
