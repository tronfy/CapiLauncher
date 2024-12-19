import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'
import RegisterPage from './pages/register'
import LaunchPage from './pages/launch'
import SettingsPage from './pages/settings'

function App() {
  async function get_nick(): Promise<string> {
    return await invoke('get_nick')
  }

  async function log(message: string) {
    return await invoke('log', { message })
  }

  const [nickname, setNickname] = useState<string | null>(null)
  const [settingsOpen, setSettingsOpen] = useState<boolean>(false)

  async function saveNickname(nickname: string) {
    const nick = nickname.trim()
    log(nick)
    invoke('save_nick', { nickname: nick }).then(() => {
      log('saved')
      setNickname(nick)
    })
  }

  async function launch() {
    await invoke('launch')
  }

  // try to get saved nickname
  useEffect(() => {
    get_nick().then(nickname => {
      if (nickname !== null) setNickname(nickname)
    })
  }, [])

  if (nickname === null || nickname === '') {
    return <RegisterPage saveNickname={saveNickname} />
  }

  if (settingsOpen) {
    return <SettingsPage closeSettings={() => setSettingsOpen(false)} resetNick={() => setNickname(null)} />
  }

  return (
    <LaunchPage
      nickname={nickname}
      launch={launch}
      launching={false}
      openSettings={() => {
        setSettingsOpen(true)
      }}
    />
  )
}

export default App
