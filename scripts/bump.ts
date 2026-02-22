import fs from 'fs'
import path from 'path'

const pkgPath = path.resolve(process.cwd(), 'package.json')
const cargoPath = path.resolve(process.cwd(), 'src-tauri', 'Cargo.toml')
const tauriConfigPath = path.resolve(process.cwd(), 'src-tauri', 'tauri.conf.json')

type Pkg = {
  [key: string]: any
  version: string
}

const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8')) as Pkg
const bump = process.env.INPUT_BUMP || 'patch'

function inc(v: string, part: string): string {
  const p = v.split('.').map(n => parseInt(n, 10))
  if (part === 'major') { p[0]++; p[1] = 0; p[2] = 0 }
  else if (part === 'minor') { p[1]++; p[2] = 0 }
  else { p[2]++ }
  return p.join('.')
}

const newver = inc(pkg.version, bump)
pkg.version = newver
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n', 'utf8')

let cargo = fs.readFileSync(cargoPath, 'utf8')
cargo = cargo.replace(/version\s*=\s*"[^"]+"/, `version = "${newver}"`)
fs.writeFileSync(cargoPath, cargo, 'utf8')

const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf8')) as Pkg
tauriConfig.version = newver
fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2) + '\n', 'utf8')

console.log(newver)
