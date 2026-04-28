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
  /** Si true et `optional` true, le fichier est sélectionné par défaut au premier lancement. */
  default?: boolean
  /** Nom lisible affiché dans l'UI de gestion des mods optionnels. */
  name?: string
  /** Description affichée dans l'UI de gestion des mods optionnels. */
  description?: string
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
