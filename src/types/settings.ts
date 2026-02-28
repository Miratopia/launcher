/**
 * Requête envoyée à la commande Tauri `display_settings`.
 *
 * Permet de récupérer les paramètres associés à un modpack.
 */
export type GetSettingsRequest = {
  modpackName: string,
}

/**
 * Requête envoyée à la commande Tauri `update_settings`.
 *
 * Contient le nom du modpack ainsi que les nouveaux paramètres
 * à appliquer.
 */
export type UpdateSettingsRequest = {
  modpackName: string,
  newSettings: Settings,
}

/**
 * Paramètres configurables d'un modpack.
 *
 * Tous les champs sont optionnels afin de permettre :
 * - des mises à jour partielles
 * - un stockage flexible dans le backend
 *
 * Si un champ est `undefined`, cela signifie qu'il n'est pas défini
 * ou qu'il ne doit pas être modifié lors d'une mise à jour.
 */
export interface Settings {
  javaDistribution?: JavaDistribution,
  minMemory?: number,
  maxMemory?: number,
  fullScreen?: boolean,
  windowWidth?: number,
  windowHeight?: number,
}

/**
 * Liste des distributions Java supportées par le launcher.
 *
 * Les valeurs correspondent aux identifiants utilisés
 * côté backend pour télécharger / installer la JVM.
 */
export enum JavaDistribution {
  Temurin = "temurin",
  GraalVM = "graalvm",
  Zulu = "zulu",
  Liberica = "liberica",
}

/**
 * Élément utilisable dans une interface graphique
 * pour afficher une distribution Java.
 *
 * Typiquement utilisé dans un `select`, `dropdown`
 * ou un composant UI similaire.
 */
export interface JavaDistributionListItem {
  value: JavaDistribution,
  label: string,
}

/**
 * Paramètres par défaut utilisés lorsque
 * aucun réglage n'est encore défini pour un modpack.
 *
 * Ces valeurs servent notamment :
 * - à initialiser l'interface
 * - à fallback si le store ne contient rien
 */
export const DEFAULT_SETTINGS: Settings = {
  javaDistribution: JavaDistribution.Temurin,
  minMemory: 1024,
  maxMemory: 4096,
  fullScreen: false,
  windowWidth: 1280,
  windowHeight: 720,
}
