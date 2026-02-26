import type { ProfileConfig } from '../../types'

interface Props {
  config: ProfileConfig
  onChange: (config: ProfileConfig) => void
}

export default function WhatsAppTab({ config, onChange }: Props) {
  const channel = config.channels.find((c) => c.type === 'whatsapp')
  const enabled = !!channel

  const toggle = () => {
    if (enabled) {
      onChange({ ...config, channels: config.channels.filter((c) => c.type !== 'whatsapp') })
    } else {
      onChange({
        ...config,
        channels: [...config.channels, { type: 'whatsapp', bridge_url: 'ws://localhost:3001' }],
      })
    }
  }

  const updateBridgeUrl = (v: string) => {
    const channels = config.channels.map((c) =>
      c.type === 'whatsapp' ? { ...c, bridge_url: v } : c
    )
    onChange({ ...config, channels })
  }

  return (
    <div className="space-y-4">
      <div className="text-xs text-gray-400 space-y-1.5 bg-surface-dark/50 rounded-lg p-3 border border-gray-700/50">
        <p className="font-medium text-gray-300">WhatsApp Bridge</p>
        <p>Connect via a Node.js WebSocket bridge that wraps WhatsApp Web. Supports text messages and media.</p>
        <ol className="list-decimal list-inside space-y-0.5 text-gray-500">
          <li>Set up a WhatsApp Web bridge server (Node.js app using whatsapp-web.js or whatsmeow)</li>
          <li>Run the bridge &mdash; scan the QR code with your WhatsApp mobile app</li>
          <li>The bridge exposes a WebSocket on port 3001 (default) and media server on port 3002</li>
          <li>Point the Bridge URL below to the bridge's WebSocket endpoint</li>
        </ol>
        <p className="text-gray-600">Uses outbound WebSocket connection (no webhook URL needed). Each profile can use a separate bridge instance.</p>
      </div>

      <label className="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          checked={enabled}
          onChange={toggle}
          className="w-4 h-4 rounded bg-surface-dark border-gray-600 text-accent focus:ring-accent"
        />
        <span className="text-sm text-gray-300">Enable WhatsApp channel</span>
      </label>

      {enabled && (
        <div>
          <label className="block text-sm font-medium text-gray-300 mb-1.5">Bridge URL</label>
          <input
            value={(channel as any)?.bridge_url || 'ws://localhost:3001'}
            onChange={(e) => updateBridgeUrl(e.target.value)}
            placeholder="ws://localhost:3001"
            className="input text-xs font-mono"
          />
          <p className="text-[10px] text-gray-600 mt-1">
            WebSocket URL of the WhatsApp bridge. Default: ws://localhost:3001. Run multiple bridges on different ports for multiple profiles.
          </p>
        </div>
      )}
    </div>
  )
}
