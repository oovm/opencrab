import type { Plugin } from 'vite'
import fs from 'fs'
import path from 'path'

const DOCS_VIRTUAL_MODULE_ID = 'virtual:docs'
const RESOLVED_DOCS_VIRTUAL_MODULE_ID = '\0' + DOCS_VIRTUAL_MODULE_ID

export default function docsPlugin(): Plugin {
  return {
    name: 'vite-plugin-docs',
    resolveId(id) {
      if (id === DOCS_VIRTUAL_MODULE_ID) {
        return RESOLVED_DOCS_VIRTUAL_MODULE_ID
      }
      return null
    },
    load(id) {
      if (id === RESOLVED_DOCS_VIRTUAL_MODULE_ID) {
        const docsDir = path.resolve(__dirname, '../../documentation/zh-hans')
        
        const docs: Record<string, string> = {}
        
        function traverseDir(dir: string, relativePath: string = '') {
          const files = fs.readdirSync(dir)
          
          for (const file of files) {
            const fullPath = path.join(dir, file)
            const stat = fs.statSync(fullPath)
            
            if (stat.isDirectory()) {
              traverseDir(fullPath, path.join(relativePath, file))
            } else if (file.endsWith('.md')) {
              const content = fs.readFileSync(fullPath, 'utf-8')
              const key = path.join(relativePath, file).replace(/\\/g, '/')
              docs[key] = content
            }
          }
        }
        
        traverseDir(docsDir)
        
        return `
          export const docsModules = ${JSON.stringify(docs)}
        `
      }
      return null
    }
  }
}
