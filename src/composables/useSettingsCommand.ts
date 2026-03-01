import { invoke } from '@tauri-apps/api/core'
import { GetSettingsRequest, JavaDistribution, JavaDistributionListItem, Settings, UpdateSettingsRequest } from '../types/settings'
import consola from 'consola'

/**
 * Interface définissant les fonctions disponibles pour interagir avec les commandes Tauri liées aux paramètres d'un modpack.
 *
 * Cette interface est utilisée pour typer le résultat de la fonction `useSettingsCommand()`, qui retourne un objet contenant ces fonctions.
 *
 * Les fonctions incluent :
 * - `displaySettings(modpackName: string): Promise<Settings>` : récupère les paramètres d'un modpack donné.
 * - `updateSettings(modpackName: string, newSettings: Settings): Promise<Settings>` : met à jour les paramètres d'un modpack donné.
 * - `listJavaDistributions(): JavaDistributionListItem[]` : liste les distributions Java disponibles.
 */
export interface UseSettingsCommand {
  displayModpackSettings: (modpackName: string) => Promise<Settings>,
  updateModpackSettings: (modpackName: string, newSettings: Settings) => Promise<Settings>,
  listJavaDistributions: () => JavaDistributionListItem[],
}

/**
 * Composable Vue permettant d'interagir avec les commandes Tauri
 * liées aux paramètres d'un modpack.
 *
 * Ce composable encapsule les appels `invoke()` vers le backend Rust
 * afin de simplifier leur utilisation dans l'interface.
 *
 * Les fonctions retournées permettent de :
 * - récupérer les paramètres d'un modpack
 * - mettre à jour les paramètres d'un modpack
 * - lister les distributions Java disponibles
 *
 * Toutes les erreurs sont loggées via `consola` avant d'être propagées.
 */
export function useSettingsCommand(): UseSettingsCommand {
  /**
   * Récupère les paramètres d'un modpack depuis le backend Tauri.
   *
   * Cette fonction appelle la commande Rust `display_modpack_settings`.
   *
   * @param modpackName Nom du modpack dont on veut récupérer les paramètres.
   * @returns Les paramètres du modpack.
   * @throws Une erreur si l'appel à la commande Tauri échoue.
   */
  async function displayModpackSettings(modpackName: string): Promise<Settings> {
    try {
      return await invoke<Settings>('display_modpack_settings', <GetSettingsRequest>{ modpackName })
    } catch (error) {
      consola.error('Failed to display modpack settings:', error)
      throw error
    }
  }

  /**
   * Met à jour les paramètres d'un modpack.
   *
   * Cette fonction appelle la commande Rust `update_modpack_settings`.
   *
   * @param modpackName Nom du modpack à mettre à jour.
   * @param newSettings Nouveaux paramètres à appliquer.
   * @returns Les paramètres mis à jour.
   * @throws Une erreur si l'appel à la commande Tauri échoue.
   */
  async function updateModpackSettings(modpackName: string, newSettings: Settings): Promise<Settings> {
    try {
      return await invoke<Settings>('update_modpack_settings', <UpdateSettingsRequest>{ modpackName, newSettings })
    } catch (error) {
      consola.error('Failed to update modpack settings:', error)
      throw error
    }
  }

  /**
   * Retourne la liste des distributions Java disponibles.
   *
   * Convertit l'enum `JavaDistribution` en tableau utilisable
   * dans les composants UI (ex : select, dropdown, radio group).
   *
   * @returns Tableau d'objets `{ label, value }` représentant
   * les distributions Java disponibles.
   *
   * Exemple de résultat :
   * ```ts
   * [
   *   { label: "Temurin", value: "temurin" },
   *   { label: "Zul", value: "zulu" }
   * ]
   * ```
   */
  function listJavaDistributions(): JavaDistributionListItem[] {
    return Object.values(JavaDistribution).map((v) => ({
      label: v.charAt(0).toUpperCase() + v.slice(1),
      value: v,
    }))
  }

  return {
    displayModpackSettings,
    updateModpackSettings,
    listJavaDistributions,
  }
}
