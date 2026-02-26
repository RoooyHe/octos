import type { ProfileConfig } from '../../types'

interface Props {
  config: ProfileConfig
  onChange: (config: ProfileConfig) => void
}

export default function FeishuTab({ config, onChange }: Props) {
  const channel = config.channels.find((c) => c.type === 'feishu')
  const enabled = !!channel

  const toggle = () => {
    if (enabled) {
      onChange({ ...config, channels: config.channels.filter((c) => c.type !== 'feishu') })
    } else {
      onChange({
        ...config,
        channels: [
          ...config.channels,
          {
            type: 'feishu',
            app_id_env: 'FEISHU_APP_ID',
            app_secret_env: 'FEISHU_APP_SECRET',
            verification_token_env: 'FEISHU_VERIFICATION_TOKEN',
            encrypt_key_env: 'FEISHU_ENCRYPT_KEY',
          },
        ],
      })
    }
  }

  const updateField = (field: string, v: string) => {
    const channels = config.channels.map((c) =>
      c.type === 'feishu' ? { ...c, [field]: v } : c
    )
    onChange({ ...config, channels })
  }

  const updateEnv = (key: string, value: string) => {
    const newEnvVars = { ...config.env_vars }
    if (value) {
      newEnvVars[key] = value
    } else {
      delete newEnvVars[key]
    }
    onChange({ ...config, env_vars: newEnvVars })
  }

  return (
    <div className="space-y-4">
      <div className="text-xs text-gray-400 space-y-1.5 bg-surface-dark/50 rounded-lg p-3 border border-gray-700/50">
        <p className="font-medium text-gray-300">Feishu / Lark</p>
        <p>Connect to Feishu (China) or Lark (international) by ByteDance. Supports text, rich text, images, files, and card messages.</p>
        <ol className="list-decimal list-inside space-y-0.5 text-gray-500">
          <li>Go to <a href="https://open.feishu.cn/" target="_blank" rel="noopener" className="text-accent hover:underline">Feishu Open Platform</a> or <a href="https://open.larksuite.com/" target="_blank" rel="noopener" className="text-accent hover:underline">Lark Developer</a> and create a custom app</li>
          <li>Copy <strong>App ID</strong> and <strong>App Secret</strong> from the credentials page</li>
          <li>Under Event Subscriptions, get the <strong>Verification Token</strong> and <strong>Encrypt Key</strong></li>
          <li>Subscribe to <code className="bg-gray-800 px-1 rounded">im.message.receive_v1</code> event</li>
          <li>Add permissions: <code className="bg-gray-800 px-1 rounded">im:message</code>, <code className="bg-gray-800 px-1 rounded">im:message:send_as_bot</code></li>
        </ol>
        <p className="text-gray-600"><strong>WebSocket mode</strong> (recommended): No public URL needed, the bot connects outbound. <strong>Webhook mode</strong>: Requires a public URL (e.g., via ngrok). Each profile needs a different webhook port.</p>
      </div>

      <label className="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          checked={enabled}
          onChange={toggle}
          className="w-4 h-4 rounded bg-surface-dark border-gray-600 text-accent focus:ring-accent"
        />
        <span className="text-sm text-gray-300">Enable Feishu / Lark channel</span>
      </label>

      {enabled && (
        <>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1.5">App ID</label>
            <input
              type="password"
              value={config.env_vars['FEISHU_APP_ID'] || ''}
              onChange={(e) => updateEnv('FEISHU_APP_ID', e.target.value)}
              placeholder="cli_xxxx"
              className="input text-xs font-mono"
            />
            <p className="text-[10px] text-gray-600 mt-1">From app credentials page. Stored as FEISHU_APP_ID.</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1.5">App Secret</label>
            <input
              type="password"
              value={config.env_vars['FEISHU_APP_SECRET'] || ''}
              onChange={(e) => updateEnv('FEISHU_APP_SECRET', e.target.value)}
              placeholder="secret..."
              className="input text-xs font-mono"
            />
            <p className="text-[10px] text-gray-600 mt-1">From app credentials page. Stored as FEISHU_APP_SECRET.</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1.5">Verification Token</label>
            <input
              type="password"
              value={config.env_vars['FEISHU_VERIFICATION_TOKEN'] || ''}
              onChange={(e) => updateEnv('FEISHU_VERIFICATION_TOKEN', e.target.value)}
              placeholder="verification token"
              className="input text-xs font-mono"
            />
            <p className="text-[10px] text-gray-600 mt-1">From Event Subscriptions settings. Stored as FEISHU_VERIFICATION_TOKEN.</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1.5">Encrypt Key</label>
            <input
              type="password"
              value={config.env_vars['FEISHU_ENCRYPT_KEY'] || ''}
              onChange={(e) => updateEnv('FEISHU_ENCRYPT_KEY', e.target.value)}
              placeholder="encrypt key"
              className="input text-xs font-mono"
            />
            <p className="text-[10px] text-gray-600 mt-1">From Event Subscriptions settings. Stored as FEISHU_ENCRYPT_KEY.</p>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1.5">Mode</label>
            <select
              value={(channel as any)?.mode || 'webhook'}
              onChange={(e) => updateField('mode', e.target.value)}
              className="input text-xs"
            >
              <option value="websocket">WebSocket (recommended, no public URL needed)</option>
              <option value="webhook">Webhook (requires public URL / ngrok)</option>
            </select>
            <p className="text-[10px] text-gray-600 mt-1">
              WebSocket connects outbound (no port conflicts). Webhook requires a unique port per profile and a public URL.
            </p>
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1.5">Region</label>
              <select
                value={(channel as any)?.region || 'feishu'}
                onChange={(e) => updateField('region', e.target.value)}
                className="input text-xs"
              >
                <option value="feishu">Feishu (China)</option>
                <option value="lark">Lark (International)</option>
              </select>
              <p className="text-[10px] text-gray-600 mt-1">
                Determines API endpoint: feishu.cn or larksuite.com
              </p>
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1.5">Webhook Port</label>
              <input
                type="number"
                value={(channel as any)?.webhook_port || ''}
                onChange={(e) => updateField('webhook_port', e.target.value ? Number(e.target.value) as any : '')}
                placeholder="9321"
                className="input text-xs"
              />
              <p className="text-[10px] text-gray-600 mt-1">
                Only for webhook mode. Use different ports per profile.
              </p>
            </div>
          </div>
        </>
      )}
    </div>
  )
}
