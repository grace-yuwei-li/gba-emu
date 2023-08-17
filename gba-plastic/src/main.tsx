import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'
import GbaProvider from './Gba.tsx'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <GbaProvider>
        <App />
    </GbaProvider>
  </React.StrictMode>,
)
