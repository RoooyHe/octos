import type { ProfileConfig } from '../../types'

const SEARCH_KEYS = [
  { key: 'PERPLEXITY_API_KEY', label: 'Perplexity', placeholder: 'pplx-...' },
  { key: 'BRAVE_SEARCH_API_KEY', label: 'Brave Search', placeholder: 'BSA...' },
  { key: 'YOU_API_KEY', label: 'You.com', placeholder: '' },
]

interface Props {
  config: ProfileConfig
  onChange: (config: ProfileConfig) => void
}

export default function SearchApiTab({ config, onChange }: Props) {
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
        <p className="font-medium text-gray-300">Web Search APIs</p>
        <p>Configure API keys for web search providers used by the <code className="bg-gray-800 px-1 rounded">web_search</code> tool. DuckDuckGo is used by default with no API key. Adding a key here enables higher-quality results.</p>
        <ul className="list-disc list-inside space-y-0.5 text-gray-500">
          <li><strong>Perplexity</strong> &mdash; <a href="https://www.perplexity.ai/settings/api" target="_blank" rel="noopener" className="text-accent hover:underline">Get API key</a> (AI-powered search)</li>
          <li><strong>Brave Search</strong> &mdash; <a href="https://brave.com/search/api/" target="_blank" rel="noopener" className="text-accent hover:underline">Get API key</a> (independent web index)</li>
          <li><strong>You.com</strong> &mdash; <a href="https://you.com/search?q=api" target="_blank" rel="noopener" className="text-accent hover:underline">Get API key</a></li>
        </ul>
      </div>

      {SEARCH_KEYS.map(({ key, label, placeholder }) => (
        <div key={key}>
          <label className="block text-sm font-medium text-gray-300 mb-1.5">{label}</label>
          <input
            type="password"
            value={config.env_vars[key] || ''}
            onChange={(e) => updateEnv(key, e.target.value)}
            placeholder={placeholder || 'API key'}
            className="input font-mono text-xs"
          />
          <p className="text-[10px] text-gray-600 mt-1">{key}</p>
        </div>
      ))}
    </div>
  )
}
