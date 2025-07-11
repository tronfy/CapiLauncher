import { invoke } from '@tauri-apps/api/core'

import logo from '../assets/logo.png'

export type HeaderProps = {
  showSettings?: boolean
  openSettings?: () => void
}

export default function Header({ showSettings, openSettings }: HeaderProps) {
  return (
    <header className="flex justify-between">
      <div className="flex items-center gap-4 pb-8">
        <img src={logo} alt="logo" className="w-16 h-16 rounded-lg" />
        <div>
          <div className="flex items-end gap-2">
            <h1 className="text-3xl font-bold">CapiLauncher</h1>
            <h2 className="text-zinc-400 pb-1">v0.3.1</h2>
          </div>
          <h2 className="text-sm">by CapivaraManca</h2>
        </div>
      </div>
      {showSettings && (
        <div className="flex h-fit gap-3">
          <button
            onClick={() => {
              invoke('open_folder')
            }}
            className="p-2"
            title="Abrir pasta"
          >
            <span className="text-xl">📂</span>
          </button>
          <button
            onClick={() => {
              openSettings && openSettings()
            }}
            className="p-2"
            title="Configurações"
          >
            <span className="text-xl">🔧</span>
          </button>
        </div>
      )}
    </header>
  )
}
