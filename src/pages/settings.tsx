import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'

export type SettingsPageProps = {
  closeSettings: () => void
  resetNick: () => void
}

export default function SettingsPage({ closeSettings, resetNick }: SettingsPageProps) {
  const [loading, setLoading] = useState<boolean>(true)
  const [systemRam, setSystemRam] = useState<number>(0)
  const [ram, setRam] = useState<number>(0)

  async function get_data() {
    invoke('get_sys_ram').then(sys_ram => {
      setSystemRam(sys_ram as number)

      invoke('get_ram').then(ram => {
        setRam(ram as number)
        setLoading(false)
      })
    })
  }

  async function set_ram(ram: number) {
    await invoke('set_ram', { ram })
  }

  useEffect(() => {
    get_data()
  }, [])

  if (loading) {
    return (
      <>
        <h1 className="text-2xl pb-2">Configurações</h1>
        <button onClick={closeSettings}> Voltar </button>

        <p className="pt-5">Carregando...</p>
      </>
    )
  }

  return (
    <div>
      <h1 className="text-2xl pb-2">Configurações</h1>
      <button onClick={closeSettings}> Voltar </button>

      <div className="pt-10">
        <label className="pr-5" htmlFor="ram">
          Memória (MB)
        </label>
        <input
          type="number"
          id="ram"
          value={ram}
          onChange={e => {
            let value = parseInt(e.target.value)
            if (value < 0) value = 0
            if (value > systemRam) value = systemRam
            setRam(value)
          }}
        />

        <button
          onClick={() => {
            setLoading(true)
            set_ram(ram).then(() => {
              closeSettings()
            })
          }}
          className="ml-3"
        >
          Salvar
        </button>
      </div>

      <p className="pt-16">Alterar nickname (isso faz você perder seus itens e progresso!)</p>

      <button
        onClick={() => {
          resetNick()
          closeSettings()
        }}
        className="bg-orange-800"
      >
        Resetar nickname
      </button>
    </div>
  )
}
