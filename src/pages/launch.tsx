import { listen } from '@tauri-apps/api/event'
import { useState } from 'react'
import Header from '../components/Header'

export type LaunchPageProps = {
  nickname: string
  launch: () => void
  openSettings?: () => void
}

export default function LaunchPage({ nickname, launch, openSettings }: LaunchPageProps) {
  const [status, setStatus] = useState<string>('')
  const [launching, setLaunching] = useState<boolean>(false)

  listen<string>('msg', event => {
    setStatus(event.payload)
  })

  return (
    <>
      <Header showSettings={!launching} openSettings={openSettings} />

      <div className="flex flex-col justify-center items-center">
        <h1 className="text-3xl pb-1">Olá, {nickname}!</h1>

        <p className="pb-10">
          Pronto para iniciar <b>CapivaraSMP XI</b>!
        </p>

        {status && <p className="text-center outline-2 text-xl font-mono font-bold text-zinc-400 pb-2">{status}...</p>}

        {launching ? (
          <button disabled className="flex items-center gap-2 px-14 py-5 cursor-default bg-zinc-800 hover:bg-zinc-800">
            <span className="text-3xl">🚀</span>
            <span className="text-2xl font-bold">Iniciando...</span>
          </button>
        ) : (
          <button
            onClick={() => {
              setLaunching(true)
              launch()
            }}
            className="flex items-center gap-2 px-14 py-5"
          >
            <span className="text-3xl">🚀</span>
            <span className="text-2xl font-bold">Iniciar</span>
          </button>
        )}
      </div>
    </>
  )
}
