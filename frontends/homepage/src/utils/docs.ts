export interface DocNode {
  id: string
  title: string
  path: string
  isDirectory: boolean
  children?: DocNode[]
  parentId?: string
  order?: number
}

export interface DocMetadata {
  title?: string
  order?: number
}

const docsModules = import.meta.glob('../documentation/zh-hans/**/*.md', { 
  query: '?raw', 
  import: 'default', 
  eager: true 
})

function parseDocMetadata(content: string): DocMetadata {
  const metadata: DocMetadata = {}
  const frontmatterMatch = content.match(/^---\n([\s\S]*?)\n---/)
  
  if (frontmatterMatch) {
    const frontmatter = frontmatterMatch[1]
    const titleMatch = frontmatter.match(/^title:\s*(.+)$/m)
    const orderMatch = frontmatter.match(/^order:\s*(\d+)$/m)
    
    if (titleMatch) {
      metadata.title = titleMatch[1].trim().replace(/^['"]|['"]$/g, '')
    }
    if (orderMatch) {
      metadata.order = parseInt(orderMatch[1], 10)
    }
  }
  
  return metadata
}

function generateId(path: string): string {
  return path
    .replace('../documentation/zh-hans/', '')
    .replace(/\.md$/, '')
    .replace(/\//g, '-')
}

function getDocTitle(path: string, metadata: DocMetadata): string {
  if (metadata.title) {
    return metadata.title
  }
  
  const filename = path.split('/').pop()?.replace('.md', '') || ''
  
  const titleMap: Record<string, string> = {
    'index': '首页',
    'introduction': '介绍',
    'quick-start': '快速开始',
    'features': '功能特性',
    'agent': '智能体',
    'capabilities': '能力',
    'chat': '对话',
    'memory': '记忆',
    'scheduler': '调度器',
    'skills': '技能',
    'tool': '工具',
    'workspace': '工作区',
    'extensibility': '可扩展性',
    'performance': '性能',
    'security': '安全性',
    'agent-core': '智能体核心',
    'architecture': '架构',
    'core-layer': '核心层',
    'decentralization': '去中心化',
    'ecosystem-overview': '生态系统概览',
    'infrastructure': '基础设施',
    'master-plan': '总体规划',
    'protocol-layer': '协议层',
    'data-models': '数据模型',
    'technology-choices': '技术选择',
    'skynet': '天网',
    'messages': '消息',
    'profile': '配置',
    'resources': '资源',
    'subnets': '子网',
    'threat-model': '威胁模型',
    'uri': 'URI',
    'add-skills': '添加技能',
    'configure-agent': '配置智能体',
    'getting-started': '开始使用',
    'use-tools': '使用工具',
    'best-practices': '最佳实践',
    'development-helper': '开发助手',
    'knowledge-base': '知识库',
    'personal-assistant': '个人助手',
    'task-automation': '任务自动化'
  }
  
  return titleMap[filename] || filename
}

function buildDocTree(docs: DocNode[]): DocNode[] {
  const tree: DocNode[] = []
  const nodeMap: Record<string, DocNode> = {}
  
  docs.forEach(doc => {
    nodeMap[doc.id] = { ...doc, children: [] }
  })
  
  docs.forEach(doc => {
    const node = nodeMap[doc.id]
    const pathParts = doc.path.split('/')
    
    if (pathParts.length <= 3) {
      tree.push(node)
    } else {
      const parentPathParts = pathParts.slice(0, -1)
      let currentParent: DocNode | undefined
      
      for (let i = parentPathParts.length; i >= 3; i--) {
        const currentParentPathParts = parentPathParts.slice(0, i)
        const parentPath = currentParentPathParts.join('/') + '/index.md'
        const parentId = generateId(parentPath)
        const parent = nodeMap[parentId]
        
        if (parent) {
          currentParent = parent
          break
        }
      }
      
      if (currentParent && currentParent.children) {
        currentParent.children.push(node)
      } else {
        tree.push(node)
      }
    }
  })
  
  const sortNodes = (nodes: DocNode[]): DocNode[] => {
    return nodes
      .sort((a, b) => (a.order || 999) - (b.order || 999))
      .map(node => ({
        ...node,
        children: node.children ? sortNodes(node.children) : undefined
      }))
  }
  
  return sortNodes(tree)
}

export async function loadDocs(): Promise<DocNode[]> {
  console.log('docsModules keys:', Object.keys(docsModules))
  const docs: DocNode[] = []
  
  for (const path in docsModules) {
    const content = docsModules[path] as string
    const metadata = parseDocMetadata(content)
    const id = generateId(path)
    const title = getDocTitle(path, metadata)
    
    docs.push({
      id,
      title,
      path,
      isDirectory: false,
      order: metadata.order
    })
  }
  
  console.log('Loaded docs:', docs)
  return buildDocTree(docs)
}

export async function getDocContent(path: string): Promise<string> {
  return docsModules[path] as string || ''
}
