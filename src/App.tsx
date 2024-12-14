import { invoke } from '@tauri-apps/api/core'
import './App.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

function App() {
  async function get_nick(): Promise<string> {
    return await invoke('get_nick')
  }

  async function save_nick(nickname: string) {
    return await invoke('save_nick', { nickname })
  }

  async function log(message: string) {
    return await invoke('log', { message })
  }

  const [loading, setLoading] = useState<boolean>(false)
  const [nickname, setNickname] = useState<string | null>(null)

  async function launch() {
    if (loading) {
      return
    }
    setLoading(true)
    await invoke('launch')
  }

  const [msg, setMsg] = useState<string>('')

  listen<string>('msg', event => {
    // alert(event.payload)
    setMsg(event.payload)
  })

  useEffect(() => {
    get_nick().then(nickname => {
      log(nickname)
      if (nickname !== null) {
        setNickname(nickname)
      }
    })
  }, [])

  if (nickname === null || nickname === '') {
    return (
      <main className="container">
        <h1>Boas vindas!</h1>

        <br />

        <p>Insira seu nome de jogador do Minecraft</p>

        <input type="text" id="nickname" placeholder="Nickname" />
        <button
          onClick={() => {
            const input = (document.getElementById('nickname') as HTMLInputElement).value

            log(input)

            save_nick(input).then(() => {
              setNickname(input)
            })
          }}
          style={{ marginLeft: '.5em' }}
        >
          Continuar
        </button>
      </main>
    )
  }

  // if (nickname !== null && !authorized) {
  //   let cmd = `/link ${nickname}`
  //   return (
  //     <main className="container">
  //       <h1>Boas vindas, {nickname}!</h1>

  //       <br />

  //       <p>digite esta mensagem no Discord do CapivaraSMP</p>

  //       <div>
  //         <span className="code">{cmd}</span>
  //       </div>

  //       <br />

  //       <p>e insira o token recebido por mensagem privada</p>

  //       <input type="text" id="token" placeholder="Token" />

  //       <button
  //         onClick={() => {
  //           const input = (document.getElementById('token') as HTMLInputElement).value

  //           if (input === '') {
  //             return
  //           }

  //           invoke('authorize', { token: input }).then(res => {
  //             if (res) {
  //               setAuthorized(true)
  //             }
  //           })
  //         }}
  //         style={{ marginLeft: '.5em' }}
  //       >
  //         Autenticar
  //       </button>
  //     </main>
  //   )
  // }

  return (
    <main className="container">
      <h1>Olá, {nickname}!</h1>

      <br />

      <p>{msg}...</p>

      {!loading && <button onClick={launch}>Iniciar</button>}
    </main>
  )
}

export default App
