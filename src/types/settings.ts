export interface Settings {
  javaDistribution?: JavaDistribution,
  minMemory?: number,
  maxMemory?: number,
  fullScreen?: boolean,
  windowWidth?: number,
  windowHeight?: number,
}

export enum JavaDistribution {
  Temurin = "temurin",
  GraalVM = "graalvm",
  Zulu = "zulu",
  Liberica = "liberica",
}

export const DEFAULT_SETTINGS: Settings = {
  javaDistribution: JavaDistribution.Temurin,
  minMemory: 1024,
  maxMemory: 4096,
  fullScreen: false,
  windowWidth: 1280,
  windowHeight: 720,
}
