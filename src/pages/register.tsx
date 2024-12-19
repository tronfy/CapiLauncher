import { useState } from 'react'
import Header from '../components/Header'

export type RegisterPageProps = {
  saveNickname: (nickname: string) => void
}

export default function RegisterPage({ saveNickname }: RegisterPageProps) {
  const [msg, setMsg] = useState('')

  return (
    <>
      <Header />

      <div className="flex flex-col justify-center items-center">
        <h1 className="text-3xl pb-5">Boas vindas!</h1>

        <p className="text-lg">Insira seu nome de jogador do Minecraft</p>
        <p>Este será o seu nick no servidor, e será visto pelos outros players.</p>

        <input type="text" id="nickname" placeholder="Nickname" className="font-mono text-2xl mt-5 mb-2" />
        <p className="mb-5 text-orange-500">{msg}</p>
        <button
          className="ml-5 flex items-center gap-2"
          onClick={() => {
            const input = (document.getElementById('nickname') as HTMLInputElement).value

            setMsg('')

            // at least 3 characters
            if (input.length < 3) {
              setMsg('O nickname deve ter pelo menos 3 caracteres!')
              return
            }

            // at most 16 characters
            if (input.length > 16) {
              setMsg('O nickname deve ter no máximo 16 caracteres!')
              return
            }

            // only letters, numbers, and underscores
            if (!/^[a-zA-Z0-9_]*$/.test(input)) {
              setMsg('O nickname deve conter apenas letras, números e underscores!')
              return
            }

            saveNickname(input)
          }}
        >
          <span className="text-xl">🔑</span>
          <span>Registrar</span>
        </button>
      </div>
    </>
  )
}
