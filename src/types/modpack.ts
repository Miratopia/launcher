export interface MinecraftModpackInfo {
  version: string
  recommendedMemory: number
}

export interface LoaderModpackInfo {
  type: string
  version: string
}

export interface FileModpackInfo {
  url: string
  path: string
  
  hash?: string
  size?: number
  optional: boolean
}

export interface ModpackInfo {
  id: string
  name: string
  default: boolean
  description: string
  hidden: boolean
  minecraft: MinecraftModpackInfo
  loaders: LoaderModpackInfo[]
  files: FileModpackInfo[]
  whitelist?: string[]
  ignoredFiles?: string[]
}
