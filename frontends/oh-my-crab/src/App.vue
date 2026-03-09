<template>
  <div class="w-full h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white">
    <div v-if="!isAuthenticated" class="flex items-center justify-center w-full h-full p-6">
      <div class="w-full max-w-md bg-white/10 backdrop-blur-xl rounded-3xl border border-white/20 p-12 shadow-2xl">
        <div class="text-center mb-10">
          <div class="w-24 h-24 mx-auto mb-6 bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 rounded-2xl shadow-lg shadow-purple-500/40"></div>
          <h1 class="text-4xl font-extrabold bg-gradient-to-r from-cyan-400 to-pink-500 bg-clip-text text-transparent mb-2">Oh My Crab</h1>
          <p class="text-white/70 text-lg">你的个人 AI 助手</p>
        </div>
        <div class="space-y-4 mb-5">
          <input
            v-model="authToken"
            type="password"
            placeholder="输入网关令牌"
            class="w-full px-5 py-4 bg-white/5 border border-white/20 rounded-xl text-white placeholder-white/40 focus:outline-none focus:border-cyan-400 focus:ring-2 focus:ring-cyan-400/30 transition-all"
            @keyup.enter="authenticate"
          />
          <button
            @click="authenticate"
            :disabled="isLoading"
            class="w-full py-4 bg-gradient-to-r from-cyan-500 via-purple-500 to-pink-500 rounded-xl font-bold text-lg cursor-pointer hover:translate-y-[-2px] hover:shadow-xl shadow-purple-500/40 transition-all disabled:opacity-60 disabled:cursor-not-allowed"
          >
            {{ isLoading ? '连接中...' : '连接' }}
          </button>
        </div>
        <p v-if="authError" class="text-pink-400 text-center text-sm mb-5">{{ authError }}</p>
        <div class="text-center pt-6 border-t border-white/10">
          <p class="text-white/40 text-sm">Powered by OpenCrab</p>
        </div>
      </div>
    </div>

    <div v-else class="flex flex-col w-full h-full">
      <header class="flex items-center justify-between px-7 py-4 bg-white/5 backdrop-blur-xl border-b border-white/10">
        <div class="flex items-center gap-4">
          <div class="w-10 h-10 bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 rounded-xl shadow-lg shadow-purple-500/30"></div>
          <span class="text-xl font-extrabold bg-gradient-to-r from-cyan-400 to-pink-500 bg-clip-text text-transparent">Oh My Crab</span>
        </div>
        <nav class="flex items-center gap-2 p-1.5 bg-white/5 rounded-xl">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="switchTab(tab.id)"
            :class="currentTab === tab.id ? 'px-5 py-2.5 bg-white/15 text-white rounded-lg font-medium' : 'px-5 py-2.5 text-white/60 hover:text-white hover:bg-white/5 rounded-lg font-medium transition-all'"
          >
            {{ tab.label }}
          </button>
        </nav>
        <div class="flex items-center gap-4">
          <div class="flex items-center gap-3 px-4 py-2 bg-green-500/10 rounded-full border border-green-500/20">
            <div class="w-2.5 h-2.5 bg-green-400 rounded-full animate-pulse shadow-[0_0_0_6px_rgba(34,197,94,0.15)]"></div>
            <span class="text-green-400 text-sm font-semibold">已连接</span>
          </div>
          <button @click="restart" class="w-11 h-11 bg-transparent hover:bg-white/10 rounded-xl cursor-pointer transition-all"></button>
        </div>
      </header>

      <main class="flex-1 flex overflow-hidden">
        <div v-if="currentTab === 'chat'" class="flex flex-1">
          <aside class="w-72 bg-white/5 backdrop-blur-xl border-r border-white/10 flex flex-col">
            <div class="p-5 border-b border-white/10">
              <button
                @click="createNewThread"
                class="w-full py-4 bg-gradient-to-r from-cyan-500/20 to-purple-500/20 border-2 border-cyan-400/30 text-cyan-400 font-bold rounded-xl hover:bg-cyan-500/30 hover:border-cyan-400 hover:translate-y-[-1px] cursor-pointer transition-all flex items-center justify-center"
              >
                新对话
              </button>
            </div>
            <div class="flex-1 overflow-y-auto p-4 space-y-1">
              <div class="flex items-center gap-4 px-4 py-4 bg-cyan-500/10 border border-cyan-400/25 rounded-xl cursor-pointer">
                <div class="w-10 h-10 bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 rounded-xl flex-shrink-0"></div>
                <span class="text-white font-medium">助手</span>
              </div>
              <div class="text-xs font-bold text-white/40 uppercase tracking-wider px-3 pt-4 pb-3">最近</div>
              <div class="flex items-center gap-4 px-4 py-4 hover:bg-white/5 rounded-xl cursor-pointer transition-all">
                <div class="w-10 h-10 bg-white/10 rounded-xl flex-shrink-0"></div>
                <div class="flex-1 min-w-0">
                  <span class="block text-white font-medium truncate">项目规划</span>
                  <small class="block text-white/40 text-xs mt-1">2小时前</small>
                </div>
              </div>
              <div class="flex items-center gap-4 px-4 py-4 hover:bg-white/5 rounded-xl cursor-pointer transition-all">
                <div class="w-10 h-10 bg-white/10 rounded-xl flex-shrink-0"></div>
                <div class="flex-1 min-w-0">
                  <span class="block text-white font-medium truncate">代码审查</span>
                  <small class="block text-white/40 text-xs mt-1">5小时前</small>
                </div>
              </div>
            </div>
          </aside>

          <div class="flex-1 flex flex-col">
            <div class="flex-1 overflow-y-auto p-9 space-y-7">
              <div class="flex gap-5 max-w-4xl">
                <div class="w-11 h-11 bg-gradient-to-br from-cyan-400 via-purple-500 to-pink-500 rounded-xl flex-shrink-0 shadow-lg shadow-purple-500/30"></div>
                <div class="px-6 py-5 bg-white/6 backdrop-blur-sm rounded-2xl border border-white/12 leading-relaxed">
                  <p>你好！我是你的 AI 助手。今天有什么可以帮到你的吗？</p>
                </div>
              </div>
              <div class="flex gap-5 max-w-4xl ml-auto flex-row-reverse">
                <div class="w-11 h-11 bg-gradient-to-br from-pink-400 via-purple-500 to-cyan-400 rounded-xl flex-shrink-0 shadow-lg shadow-pink-500/30"></div>
                <div class="px-6 py-5 bg-gradient-to-br from-cyan-500/20 to-purple-500/20 border border-cyan-400/25 rounded-2xl leading-relaxed">
                  <p>嗨！我想了解更多关于 OpenCrab 的信息。</p>
                </div>
              </div>
            </div>
            <div class="px-9 pb-10 pt-5 bg-gradient-to-t from-slate-900/95 via-slate-900/50 to-transparent">
              <div class="max-w-5xl mx-auto">
                <div class="flex items-end gap-4 px-4 py-4 pl-6 bg-white/6 backdrop-blur-xl rounded-2xl border border-white/12 focus-within:border-cyan-400/40 focus-within:shadow-[0_0_40px_rgba(34,211,238,0.15)] transition-all">
                  <textarea
                    v-model="message"
                    placeholder="输入你的消息..."
                    rows="1"
                    class="flex-1 border-0 outline-0 resize-none bg-transparent text-white text-base leading-6 placeholder-white/40 max-h-52"
                    @keydown.enter.prevent="sendMessage"
                  ></textarea>
                  <button
                    @click="sendMessage"
                    :disabled="!message.trim()"
                    class="w-12 h-12 bg-gradient-to-br from-cyan-500 via-purple-500 to-pink-500 rounded-xl cursor-pointer hover:scale-105 hover:shadow-xl shadow-purple-500/45 transition-all disabled:opacity-40 disabled:cursor-not-allowed flex-shrink-0"
                  ></button>
                </div>
                <p class="text-center text-white/40 text-xs mt-4">按 Enter 发送，Shift+Enter 换行</p>
              </div>
            </div>
          </div>
        </div>

        <div v-else class="flex-1 flex items-center justify-center">
          <div class="text-center">
            <div class="w-30 h-30 mx-auto mb-7 bg-white/6 rounded-2xl"></div>
            <h2 class="text-2xl font-bold text-white mb-2">{{ currentTabLabel }} 即将推出</h2>
            <p class="text-white/60 text-base">此功能正在开发中</p>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { ElMessage } from 'element-plus'

const isAuthenticated = ref(false)
const isLoading = ref(false)
const authToken = ref('')
const authError = ref('')
const currentTab = ref('chat')
const message = ref('')

const tabs = [
  { id: 'chat', label: '对话' },
  { id: 'memory', label: '记忆' },
  { id: 'jobs', label: '任务' },
  { id: 'routines', label: '例行' },
  { id: 'extensions', label: '扩展' },
  { id: 'skills', label: '技能' },
]

const currentTabLabel = computed(() => {
  return tabs.find(t => t.id === currentTab.value)?.label || ''
})

function authenticate() {
  if (!authToken.value.trim()) {
    authError.value = '请输入令牌'
    return
  }
  
  isLoading.value = true
  authError.value = ''
  
  setTimeout(() => {
    isAuthenticated.value = true
    isLoading.value = false
    ElMessage.success('欢迎使用 Oh My Crab！')
  }, 800)
}

function switchTab(tabId: string) {
  currentTab.value = tabId
}

function restart() {
  ElMessage.info('正在重启...')
}

function createNewThread() {
  ElMessage.success('新对话已创建！')
}

function sendMessage() {
  if (!message.value.trim()) return
  
  ElMessage.info('消息已发送！')
  message.value = ''
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  width: 100%;
  height: 100%;
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
}
</style>
