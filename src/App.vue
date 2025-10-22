<template>
  <div class="app-container transition-all duration-500" :class="darkMode ? 'dark' : ''">
    <!-- Shared Background Layers -->
    <div class="background-base fixed inset-0 -z-10 transition-colors duration-500" :class="darkMode ? 'bg-slate-900' : 'bg-gray-50'"></div>
    <div class="background-noise fixed inset-0 -z-10 noise-bg transition-opacity duration-500" :class="darkMode ? 'opacity-08' : 'opacity-30'"></div>

    <div class="content relative z-10 min-h-screen p-4 sm:p-8 text-gray-900 dark:text-gray-200">
      <header class="flex justify-between items-center mb-10">
        <h1 class="text-4xl font-bold bg-gradient-to-r from-purple-500 to-pink-500 dark:from-purple-400 dark:to-pink-400 bg-clip-text text-transparent">
          NotiFansly Sync
        </h1>
        <button @click="toggleDarkMode" class="theme-toggle glass-card p-3 rounded-2xl hover:scale-105 transition-all duration-300">
          <svg v-if="darkMode" class="w-6 h-6 text-yellow-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path></svg>
          <svg v-else class="w-6 h-6 text-purple-600" fill="currentColor" viewBox="0 0 20 20"><path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path></svg>
        </button>
      </header>

      <main class="max-w-3xl mx-auto space-y-10">
        <!-- WELCOME / SETUP -->
        <div v-if="viewState === 'welcome'" class="space-y-8 animate-fade-in">
          <div class="glass-card p-10 rounded-3xl">
            <div class="flex items-center mb-8">
              <div class="w-12 h-12 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center text-white font-bold mr-4">1</div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Connect Your Fansly Account</h2>
            </div>
            <p class="text-base text-gray-600 dark:text-gray-400 mb-8">
              Provide your Fansly authorization token to verify you are a creator. This token is processed locally and never stored externally.
            </p>
            <div class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Authorization Token</label>
                <div class="relative">
                  <input v-model="authToken" :type="showToken ? 'text' : 'password'" placeholder="Enter your Fansly auth token" class="glass-input w-full px-5 py-4 rounded-2xl focus:ring-2 focus:ring-purple-500"/>
                  <button @click="showToken = !showToken" class="absolute right-4 top-1/2 transform -translate-y-1/2 text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                    <svg v-if="showToken" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd"></path><path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z"></path></svg>
                    <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path><path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path></svg>
                  </button>
                </div>
              </div>
              <button @click="connectAndVerify" :disabled="!authToken || loading" class="glass-button w-full px-8 py-4 rounded-2xl font-semibold">
                <span v-if="loading">Verifying…</span>
                <span v-else>Connect & Verify</span>
              </button>
            </div>
          </div>
        </div>

        <!-- MAIN DASHBOARD -->
        <div v-if="viewState === 'dashboard' && fanslyProfile" class="space-y-10 animate-fade-in">
          <div class="glass-card p-8 sm:p-10 rounded-3xl">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">Welcome, <span class="text-purple-600 dark:text-pink-300">{{ fanslyProfile.username }}</span></h2>
            <p class="text-base text-gray-600 dark:text-gray-400 mb-6">Your account is connected. This application will now handle syncing your Fansly data for the NotiFansly bot.</p>
            <div class="flex items-center justify-between glass-card-inner p-4 sm:p-5 rounded-2xl">
              <span class="text-gray-700 dark:text-gray-300 text-sm font-medium">Your Sync Key</span>
              <div class="flex items-center space-x-3">
                <code class="glass-code px-4 py-2 rounded-md text-xs font-mono break-all">{{ syncKey.substring(0, 16) }}…</code>
                <button @click="copyToClipboard(syncKey)" class="text-purple-600 hover:text-purple-800 dark:text-pink-300 dark:hover:text-pink-100 transition-colors p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10">
                  <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path><path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path></svg>
                </button>
              </div>
            </div>
            <p class="text-xs text-center text-gray-500 dark:text-gray-400 pt-3">This key is required by the NotiFansly dashboard to enable creator features.</p>
          </div>

          <div class="glass-card p-8 sm:p-10 rounded-3xl">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Synchronization</h2>
            <div class="space-y-6">
              <div class="flex items-center justify-between p-4 glass-card-inner rounded-2xl">
                <span class="font-medium text-gray-900 dark:text-white">Auto Sync</span>
                <button @click="toggleAutoSync()" class="relative inline-flex h-7 w-12 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2" :class="autoSyncEnabled ? 'bg-purple-600' : 'bg-gray-500 dark:bg-gray-600'">
                  <span class="inline-block h-5 w-5 transform rounded-full bg-white transition-transform" :class="autoSyncEnabled ? 'translate-x-5' : 'translate-x-1'"></span>
                </button>
              </div>

              <div v-if="autoSyncEnabled" class="glass-card-inner p-4 rounded-2xl space-y-4 text-sm">
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Last sync:</span><span class="font-mono">{{ lastSyncTime || 'Never' }}</span></div>
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Next sync:</span><span class="font-mono">{{ nextSyncTime || 'Calculating…' }}</span></div>
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Interval:</span><span class="font-medium text-purple-600 dark:text-pink-300">{{ SYNC_INTERVAL_MINUTES }} minutes</span></div>
              </div>

              <button @click="manualSync" :disabled="loading" class="glass-button-secondary w-full px-6 py-4 rounded-2xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center">
                <svg v-if="loading" class="animate-spin -ml-2 mr-3 h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                <span v-if="loading">Syncing…</span>
                <span v-else>Sync Now</span>
              </button>
              <button @click="clearAllData" class="bg-red-500/10 text-red-500 hover:text-red-600 dark:text-red-400 hover:bg-red-500/20 dark:hover:text-red-300 px-6 py-4 rounded-2xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center w-full">
                <svg class="w-5 h-5 mr-3" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd"></path></svg>
                Log Out & Clear Data
              </button>
            </div>
          </div>
        </div>

        <!-- Global Error Display -->
        <div v-if="error" class="glass-card-error p-6 rounded-2xl animate-fade-in mt-8">
          <div class="flex items-start space-x-3">
            <svg class="w-6 h-6 text-red-500 dark:text-red-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path></svg>
            <div>
              <p class="font-medium text-red-800 dark:text-red-200">Error</p>
              <p class="text-sm mt-1 text-red-700 dark:text-red-300">{{ error }}</p>
            </div>
            <button @click="error = ''" class="ml-auto text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 transition-colors">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
            </button>
          </div>
        </div>

        <!-- Global Success Toast -->
        <div v-if="showSuccessToast" class="fixed top-4 right-4 glass-card p-4 rounded-2xl border-green-500/20 bg-green-500/10 animate-slide-in-right z-50">
          <div class="flex items-center space-x-3">
            <svg class="w-6 h-6 text-green-500 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path></svg>
            <span class="font-medium text-green-800 dark:text-green-200">{{ successMessage }}</span>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

// --- Constants ---
const SYNC_INTERVAL_MINUTES = 15;

// --- State ---
let unlistenSyncNow = null
let unlistenToggleAutoSync = null
const viewState = ref('loading')
const darkMode = ref(true)
const authToken = ref('')
const syncKey = ref('')
const fanslyProfile = ref(null)
const showToken = ref(false)
const loading = ref(false)
const error = ref('')
const autoSyncEnabled = ref(false)
const lastSyncTime = ref('')
const nextSyncTime = ref('')
const syncInterval = ref(null)
const showSuccessToast = ref(false)
const successMessage = ref('')

// --- Core Logic ---
async function connectAndVerify() {
  if (!authToken.value) {
    error.value = 'Please enter your authorization token.'
    return
  }
  loading.value = true
  error.value   = ''
  try {
    showSuccess('Fetching Fansly profile…')
    const profile = await invoke('get_fansly_profile', { authToken: authToken.value })
    fanslyProfile.value = profile

    showSuccess('Verifying with NotiFansly servers…')
    const response = await invoke('verify_creator_account', { fanslyProfile: profile })
    syncKey.value = response.sync_key

    localStorage.setItem('fansly_auth_token', authToken.value)
    localStorage.setItem('fansly_sync_key', syncKey.value)
    localStorage.setItem('fansly_profile', JSON.stringify(profile))

    showSuccess(`Welcome, ${profile.username}!`)
    viewState.value = 'dashboard'
  } catch (err) {
    error.value = `Connection failed: ${err}`
    fanslyProfile.value = null
    syncKey.value = ''
  } finally {
    loading.value = false
  }
}

async function performSync() {
  if (!authToken.value || !syncKey.value) {
    error.value = 'Authentication data is missing. Please log out and reconnect.'
    toggleAutoSync(false)
    return
  }
  loading.value = true
  error.value   = ''
  try {
    const results = await invoke('sync_all_creator_data', {
      authToken: authToken.value,
      syncKey:   syncKey.value
    })
    lastSyncTime.value = new Date().toLocaleTimeString()
    localStorage.setItem('last_sync_time', lastSyncTime.value)
    updateNextSyncTime()
    const summary = results.map(r => `${r.count} ${r.data_type.replace('_', ' ')}`).join(', ')
    showSuccess(`Sync complete: ${summary}`)
  } catch (err) {
    error.value = `Sync failed: ${err}`
    console.error('Sync error:', err)
  } finally {
    loading.value = false
  }
}

// --- UI & Helper Methods ---
function toggleDarkMode() {
  darkMode.value = !darkMode.value
  localStorage.setItem('darkMode', darkMode.value ? 'true' : 'false')
}

function showSuccess(message) {
  successMessage.value    = message
  showSuccessToast.value  = true
  setTimeout(() => { showSuccessToast.value = false }, 3500)
}

async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text)
    showSuccess('Copied to clipboard!')
  } catch (err) { console.error('Failed to copy: ', err) }
}

function clearAllData() {
  authToken.value      = ''
  syncKey.value        = ''
  fanslyProfile.value  = null
  toggleAutoSync(false)
  localStorage.clear()
  viewState.value = 'welcome'
  showSuccess('All data has been cleared.')
}

// --- Auto-Sync Logic ---
function manualSync() {
  performSync()
}

function toggleAutoSync(forceState = null) {
  const newState = forceState !== null ? forceState : !autoSyncEnabled.value
  autoSyncEnabled.value = newState
  localStorage.setItem('auto_sync_enabled', newState.toString())
  if (newState) {
    startAutoSync()
    if (forceState === null) showSuccess('Auto-sync enabled.')
  } else {
    stopAutoSync()
    if (forceState === null) showSuccess('Auto-sync disabled.')
  }
}

function startAutoSync() {
  if (syncInterval.value) clearInterval(syncInterval.value)
  performSync()
  syncInterval.value = setInterval(performSync, SYNC_INTERVAL_MINUTES * 60 * 1000)
  updateNextSyncTime()
}

function stopAutoSync() {
  if (syncInterval.value) clearInterval(syncInterval.value)
  syncInterval.value = null
  nextSyncTime.value = ''
}

function updateNextSyncTime() {
  if (autoSyncEnabled.value) {
    const next = new Date(Date.now() + SYNC_INTERVAL_MINUTES * 60 * 1000)
    nextSyncTime.value = next.toLocaleTimeString()
  }
}

// --- Lifecycle ---
onMounted(async () => {
  const savedAuthToken = localStorage.getItem('fansly_auth_token')
  const savedSyncKey   = localStorage.getItem('fansly_sync_key')
  const savedProfile   = localStorage.getItem('fansly_profile')
  const savedAutoSync  = localStorage.getItem('auto_sync_enabled')
  const savedDarkMode  = localStorage.getItem('darkMode')
  lastSyncTime.value = localStorage.getItem('last_sync_time') || ''

  if (savedAuthToken && savedSyncKey && savedProfile) {
    try {
      authToken.value     = savedAuthToken
      syncKey.value       = savedSyncKey
      fanslyProfile.value = JSON.parse(savedProfile)
      viewState.value     = 'dashboard'
    } catch (e) {
      console.error('Failed to parse saved data, clearing…')
      clearAllData()
    }
  } else {
    viewState.value = 'welcome'
  }

  if (savedDarkMode !== null) darkMode.value = savedDarkMode === 'true'
  if (savedAutoSync === 'true' && viewState.value === 'dashboard') {
    autoSyncEnabled.value = true
    startAutoSync()
  }

  try {
    unlistenSyncNow       = await listen('tray-sync-now', manualSync)
    unlistenToggleAutoSync = await listen('tray-toggle-auto-sync', () => toggleAutoSync())
  } catch (err) { console.error('Failed to set up tray event listeners:', err) }
})

onUnmounted(() => {
  if (unlistenSyncNow) unlistenSyncNow()
  if (unlistenToggleAutoSync) unlistenToggleAutoSync()
  stopAutoSync()
})
</script>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;800&display=swap');
.app-container { font-family: 'Inter', sans-serif; }
.noise-bg { background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 1024 1024' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E"); }
h1, h2 { line-height: 1.2; }
p, span, label { line-height: 1.6; }
.glass-card {
  @apply backdrop-blur-2xl shadow-2xl shadow-black/10;
  @apply bg-white/60 border border-black/10;
  @apply dark:bg-slate-800/50 dark:border-white/10 dark:shadow-black/20;
}
.glass-card-inner {
  @apply bg-black/5 border border-black/5;
  @apply dark:bg-white/5 dark:border-white/10;
}
.glass-card-error {
  @apply backdrop-blur-lg border border-red-500/20 bg-red-500/10;
}
.glass-input {
  @apply backdrop-blur-lg placeholder-gray-500 dark:placeholder-gray-400;
  @apply text-gray-900 bg-white/70 border border-black/15;
  @apply dark:text-white dark:bg-white/5 dark:border-white/20;
}
.glass-input:focus {
  @apply bg-white/80 border-black/20;
  @apply dark:bg-white/10 dark:border-white/30 outline-none;
}
.glass-button {
  @apply backdrop-blur-lg bg-gradient-to-r from-purple-600 to-pink-600 border-transparent text-white shadow-xl shadow-purple-500/20;
  @apply hover:shadow-2xl hover:shadow-purple-500/30;
}
.glass-button-secondary {
  @apply backdrop-blur-lg shadow-lg shadow-black/5;
  @apply bg-black/5 border border-black/10 text-gray-800;
  @apply dark:bg-white/10 dark:border-white/15 dark:text-white dark:shadow-black/20;
}
.glass-button-secondary:hover {
  @apply bg-black/10 shadow-xl;
  @apply dark:bg-white/15;
}
.glass-code {
  @apply backdrop-blur-lg border;
  @apply bg-black/5 border-black/10 text-gray-800;
  @apply dark:bg-black/20 dark:border-white/10 dark:text-gray-200;
}
@keyframes fade-in { from { opacity: 0; transform: translateY(1rem); } to { opacity: 1; transform: translateY(0); } }
@keyframes slide-in-right { from { opacity: 0; transform: translateX(2rem); } to { opacity: 1; transform: translateX(0); } }
.animate-fade-in { animation: fade-in 0.5s ease-out; }
.animate-slide-in-right { animation: slide-in-right 0.4s cubic-bezier(0.25, 1, 0.5, 1); }
.dark { color-scheme: dark; }
::-webkit-scrollbar { width: 10px; }
::-webkit-scrollbar-track { background: rgba(0,0,0,0.05); }
.dark ::-webkit-scrollbar-track { background: rgba(255,255,255,0.05); }
::-webkit-scrollbar-thumb { background: rgba(0,0,0,0.2); border-radius: 5px; }
.dark ::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.2); }
::-webkit-scrollbar-thumb:hover { background: rgba(0,0,0,0.3); }
.dark ::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.3); }
.opacity-08 { opacity: 0.08; }
</style>
