<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 via-gray-50 to-slate-100">
    <header class="bg-white shadow-sm border-b border-gray-200 sticky top-0 z-50">
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-[#b7410e] rounded-lg flex items-center justify-center">
              <span class="text-white font-bold text-xl">C</span>
            </div>
            <span class="text-2xl font-bold text-[#b7410e]">OpenCrab</span>
          </div>
          
          <nav class="flex items-center gap-8">
            <button 
              @click="currentSection = 'home'"
              :class="[
                'px-4 py-2 rounded-lg font-medium transition-all',
                currentSection === 'home' ? 'text-[#b7410e] bg-[#b7410e]/10' : 'text-gray-600 hover:text-[#b7410e] hover:bg-gray-100'
              ]"
            >
              Home
            </button>
            <button 
              @click="currentSection = 'download'"
              :class="[
                'px-4 py-2 rounded-lg font-medium transition-all',
                currentSection === 'download' ? 'text-[#b7410e] bg-[#b7410e]/10' : 'text-gray-600 hover:text-[#b7410e] hover:bg-gray-100'
              ]"
            >
              Download
            </button>
            <button 
              @click="currentSection = 'docs'"
              :class="[
                'px-4 py-2 rounded-lg font-medium transition-all',
                currentSection === 'docs' ? 'text-[#b7410e] bg-[#b7410e]/10' : 'text-gray-600 hover:text-[#b7410e] hover:bg-gray-100'
              ]"
            >
              Documentation
            </button>
          </nav>
        </div>
      </div>
    </header>

    <div v-if="currentSection === 'home'" class="container mx-auto px-4 py-16">
      <div class="text-center max-w-4xl mx-auto">
        <h1 class="text-5xl font-extrabold text-gray-900 mb-6">
          下一代 AI 协作平台
        </h1>
        <p class="text-xl text-gray-600 mb-8">
          OpenCrab 提供强大的 AI 智能体协作能力，让您的工作更高效、更智能。
        </p>
        <div class="flex gap-4 justify-center">
          <button 
            @click="currentSection = 'download'"
            class="px-8 py-4 bg-[#b7410e] text-white rounded-xl font-bold text-lg hover:bg-[#9c3a0d] transition-all shadow-lg shadow-[#b7410e]/20"
          >
            立即下载
          </button>
          <button 
            @click="currentSection = 'docs'"
            class="px-8 py-4 border-2 border-[#b7410e] text-[#b7410e] rounded-xl font-bold text-lg hover:bg-[#b7410e]/10 transition-all"
          >
            查看文档
          </button>
        </div>
      </div>

      <div class="mt-20 grid grid-cols-3 gap-8">
        <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8 text-center">
          <div class="w-16 h-16 bg-[#b7410e]/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
            <span class="text-3xl">🤖</span>
          </div>
          <h3 class="text-xl font-bold text-gray-900 mb-3">智能协作</h3>
          <p class="text-gray-600">多个 AI 智能体协同工作，完成复杂任务</p>
        </div>
        <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8 text-center">
          <div class="w-16 h-16 bg-[#b7410e]/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
            <span class="text-3xl">⚡</span>
          </div>
          <h3 class="text-xl font-bold text-gray-900 mb-3">高性能</h3>
          <p class="text-gray-600">基于 Rust 构建，速度极快</p>
        </div>
        <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8 text-center">
          <div class="w-16 h-16 bg-[#b7410e]/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
            <span class="text-3xl">🔒</span>
          </div>
          <h3 class="text-xl font-bold text-gray-900 mb-3">安全可靠</h3>
          <p class="text-gray-600">去中心化设计，数据安全有保障</p>
        </div>
      </div>
    </div>

    <div v-if="currentSection === 'download'" class="container mx-auto px-4 py-16">
      <div class="max-w-4xl mx-auto">
        <h1 class="text-4xl font-extrabold text-gray-900 mb-8 text-center">下载 OpenCrab</h1>
        
        <div class="grid grid-cols-2 gap-8">
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 bg-blue-100 rounded-xl flex items-center justify-center">
                <span class="text-2xl">🪟</span>
              </div>
              <div>
                <h3 class="text-xl font-bold text-gray-900">Windows</h3>
                <p class="text-gray-500">Windows 10/11</p>
              </div>
            </div>
            <button class="w-full py-3 bg-[#b7410e] text-white rounded-xl font-bold hover:bg-[#9c3a0d] transition-all">
              下载 Windows 版本
            </button>
          </div>
          
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 bg-gray-100 rounded-xl flex items-center justify-center">
                <span class="text-2xl">🍎</span>
              </div>
              <div>
                <h3 class="text-xl font-bold text-gray-900">macOS</h3>
                <p class="text-gray-500">macOS 11+</p>
              </div>
            </div>
            <button class="w-full py-3 bg-[#b7410e] text-white rounded-xl font-bold hover:bg-[#9c3a0d] transition-all">
              下载 macOS 版本
            </button>
          </div>
          
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 bg-yellow-100 rounded-xl flex items-center justify-center">
                <span class="text-2xl">🐧</span>
              </div>
              <div>
                <h3 class="text-xl font-bold text-gray-900">Linux</h3>
                <p class="text-gray-500">Ubuntu, Fedora, etc.</p>
              </div>
            </div>
            <button class="w-full py-3 bg-[#b7410e] text-white rounded-xl font-bold hover:bg-[#9c3a0d] transition-all">
              下载 Linux 版本
            </button>
          </div>
          
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 bg-purple-100 rounded-xl flex items-center justify-center">
                <span class="text-2xl">📦</span>
              </div>
              <div>
                <h3 class="text-xl font-bold text-gray-900">源码</h3>
                <p class="text-gray-500">GitHub 仓库</p>
              </div>
            </div>
            <button class="w-full py-3 bg-gray-800 text-white rounded-xl font-bold hover:bg-gray-700 transition-all">
              查看 GitHub
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="currentSection === 'docs'" class="container mx-auto px-4 py-8">
      <div class="flex gap-8">
        <aside class="w-80 flex-shrink-0">
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-6 sticky top-24">
            <h2 class="text-xl font-bold text-gray-800 mb-4">文档导航</h2>
            <nav v-if="docTree.length > 0">
              <div v-for="node in docTree" :key="node.id" class="mb-2">
                <DocTreeNode :node="node" :current-path="currentDocPath" @select="selectDoc" />
              </div>
            </nav>
            <div v-else class="text-gray-500 text-sm">
              加载文档中...
            </div>
          </div>
        </aside>

        <main class="flex-1">
          <div class="bg-white rounded-2xl shadow-lg border border-gray-200 p-8">
            <div v-if="currentDoc">
              <h2 class="text-3xl font-bold mb-6 text-gray-800">{{ currentDoc.title }}</h2>
              <MarkdownViewer :content="currentDocContent" />
            </div>
            <div v-else class="text-gray-500 text-center py-12">
              <p class="text-lg">请从左侧选择文档查看</p>
            </div>
          </div>
        </main>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { loadDocs, getDocContent, type DocNode } from '@/docs'
import MarkdownViewer from '@/components/MarkdownViewer.vue'
import DocTreeNode from '@/components/DocTreeNode.vue'

const currentSection = ref<'home' | 'download' | 'docs'>('home')
const docTree = ref<DocNode[]>([])
const currentDocPath = ref<string>('')
const currentDoc = ref<DocNode | null>(null)
const currentDocContent = ref<string>('')

async function init() {
  console.log('Initializing docs...')
  docTree.value = await loadDocs()
  console.log('Loaded docTree:', docTree.value)
  if (docTree.value.length > 0) {
    const firstDoc = findFirstDoc(docTree.value[0])
    if (firstDoc) {
      selectDoc(firstDoc)
    }
  }
}

function findFirstDoc(node: DocNode): DocNode | null {
  if (!node.isDirectory && !node.children?.length) {
    return node
  }
  if (node.children && node.children.length > 0) {
    return findFirstDoc(node.children[0])
  }
  return null
}

async function selectDoc(node: DocNode) {
  currentDocPath.value = node.path
  currentDoc.value = node
  currentDocContent.value = await getDocContent(node.path)
}

onMounted(() => {
  init()
})
</script>
