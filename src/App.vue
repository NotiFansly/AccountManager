<template>
  <div class="min-h-screen transition-all duration-500" :class="darkMode ? 'dark' : ''">
    <!-- Shared Background Elements -->
    <div 
      class="fixed inset-0 -z-10 transition-colors duration-500"
      :class="darkMode ? 'bg-slate-900' : 'bg-gray-100'"
    ></div>
    <!-- Subtle noise texture overlay -->
    <div 
      class="fixed inset-0 -z-10 noise-bg transition-opacity duration-500"
      :class="darkMode ? 'opacity-10' : 'opacity-40'"
    ></div>

    <!-- PRIMARY FIX HERE -->
    <div class="relative z-10 min-h-screen p-4 sm:p-8 text-gray-900 dark:text-gray-200">
      <header class="flex justify-between items-center mb-8">
        <h1 class="text-4xl font-bold bg-gradient-to-r from-purple-500 to-pink-500 dark:from-purple-400 dark:to-pink-400 bg-clip-text text-transparent">
          Fansly Account Manager
        </h1>
        <button @click="toggleDarkMode" class="glass-card p-3 rounded-xl hover:scale-105 transition-all duration-300">
          <!-- Dark/Light Mode SVGs -->
          <svg v-if="darkMode" class="w-6 h-6 text-yellow-400" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path></svg>
          <svg v-else class="w-6 h-6 text-purple-600" fill="currentColor" viewBox="0 0 20 20"><path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path></svg>
        </button>
      </header>

      <main class="max-w-4xl mx-auto">
        <!-- WELCOME / SETUP -->
        <div v-if="viewState === 'welcome'" class="space-y-8 animate-fade-in">
          <!-- Step 1: Fetch Fansly Data -->
          <div class="glass-card p-8 rounded-2xl">
            <div class="flex items-center mb-6">
              <div class="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center text-white font-bold mr-4">1</div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Connect Your Fansly Account</h2>
            </div>
            <div class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-600 dark:text-gray-300 mb-3">Authorization Token</label>
                <div class="relative">
                  <input v-model="authToken" :type="showToken ? 'text' : 'password'" placeholder="Enter your Fansly auth token" class="glass-input w-full px-4 py-3 rounded-xl focus:ring-2 focus:ring-purple-500"/>
                  <button @click="showToken = !showToken" class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white">
                    <svg v-if="showToken" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd"></path><path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z"></path></svg>
                    <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20"><path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path><path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path></svg>
                  </button>
                </div>
              </div>
              <button @click="fetchFanslyData" :disabled="!authToken || loading" class="glass-button px-8 py-3 rounded-xl font-semibold">
                <span v-if="loading">Fetching...</span>
                <span v-else>Fetch Data</span>
              </button>
            </div>
          </div>

          <div v-if="fanslyData" class="glass-card p-8 rounded-2xl animate-fade-in">
             <div class="flex items-center mb-6">
              <div class="w-10 h-10 bg-gradient-to-r from-green-500 to-emerald-500 rounded-full flex items-center justify-center text-white font-bold mr-4">2</div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Link to Dashboard</h2>
            </div>
            <div class="space-y-4">
              <p class="text-gray-700 dark:text-gray-200">Your Fansly profile <strong class="text-gray-900 dark:text-white">{{ fanslyData.username }}</strong> is ready. Create an account for the dashboard to start syncing.</p>
              <button @click="createAccount" :disabled="loading" class="glass-button px-8 py-3 rounded-xl font-semibold">
                <span v-if="loading">Creating...</span>
                <span v-else>Create / Link Account</span>
              </button>
              <p class="text-xs text-center text-gray-500 dark:text-gray-400 pt-2">
                If an account for this Fansly user already exists, this will reset its password and provide you with new credentials.
              </p>
            </div>
          </div>
        </div>
        
        <!-- MAIN DASHBOARD -->
        <div v-if="viewState === 'dashboard' && accountData && fanslyData" class="space-y-8 animate-fade-in">
          <div class="glass-card p-6 sm:p-8 rounded-2xl">
            <div class="flex items-center space-x-4">
              <img v-if="fanslyData.avatar" :src="fanslyData.avatar" alt="Avatar" class="w-16 h-16 rounded-full border-2 border-black/10 dark:border-white/20">
              <div v-else class="w-16 h-16 rounded-full bg-gradient-to-br from-purple-500 to-pink-500 flex items-center justify-center text-white text-2xl font-bold border-2 border-black/10 dark:border-white/20">
                  {{ fanslyData.displayName?.charAt(0) }}
              </div>
              <div>
                <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{{ fanslyData.displayName }}</h2>
                <p class="text-sm text-purple-600 dark:text-purple-300">@{{ fanslyData.username }}</p>
              </div>
            </div>
            
            <div class="grid grid-cols-2 gap-4 mt-6 text-center">
              <div>
                  <p class="text-xl font-bold text-gray-900 dark:text-white">{{ fanslyData.subscriberCount?.toLocaleString() || 0 }}</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wider">Subscribers</p>
              </div>
              <div>
                  <p class="text-xl font-bold text-gray-900 dark:text-white">{{ fanslyData.followCount?.toLocaleString() || 0 }}</p>
                  <p class="text-xs text-gray-500 dark:text-gray-400 uppercase tracking-wider">Followers</p>
              </div>
            </div>

            <hr class="border-black/10 dark:border-white/10 my-6">

            <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Dashboard Credentials</h3>
            <div class="space-y-4">
              <div class="flex items-center justify-between glass-card-inner p-3 rounded-lg">
                <span class="text-gray-700 dark:text-gray-300 text-sm font-medium">Account ID</span>
                <div class="flex items-center space-x-2">
                  <code class="glass-code px-3 py-1 rounded-md text-sm font-mono">{{ accountData.account_id }}</code>
                  <button @click="copyToClipboard(accountData.account_id)" class="text-purple-600 hover:text-purple-800 dark:text-purple-400 dark:hover:text-purple-300 transition-colors p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path><path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path></svg>
                  </button>
                </div>
              </div>
              <div class="flex items-center justify-between glass-card-inner p-3 rounded-lg">
                <span class="text-gray-700 dark:text-gray-300 text-sm font-medium">Password</span>
                <div class="flex items-center space-x-2">
                  <code class="glass-code px-3 py-1 rounded-md text-sm font-mono">{{ showPassword ? accountData.password : '••••••••••••' }}</code>
                  <button @click="showPassword = !showPassword" class="text-purple-600 hover:text-purple-800 dark:text-purple-400 dark:hover:text-purple-300 transition-colors p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10">
                    <svg v-if="showPassword" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd"></path><path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z"></path></svg>
                    <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path><path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path></svg>
                  </button>
                  <button @click="copyToClipboard(accountData.password)" class="text-purple-600 hover:text-purple-800 dark:text-purple-400 dark:hover:text-purple-300 transition-colors p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path><path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path></svg>
                  </button>
                </div>
              </div>
              <div class="flex items-center justify-between glass-card-inner p-3 rounded-lg">
                <span class="text-gray-700 dark:text-gray-300 text-sm font-medium">Sync Key</span>
                <div class="flex items-center space-x-2">
                  <code class="glass-code px-3 py-1 rounded-md text-xs font-mono break-all">{{ accountData.sync_key.substring(0, 16) }}...</code>
                  <button @click="copyToClipboard(accountData.sync_key)" class="text-purple-600 hover:text-purple-800 dark:text-purple-400 dark:hover:text-purple-300 transition-colors p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path><path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path></svg>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div class="glass-card p-6 sm:p-8 rounded-2xl">
            <div class="flex items-center mb-6">
              <div class="w-10 h-10 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full flex items-center justify-center text-white mr-4">
                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path></svg>
              </div>
              <h2 class="text-2xl font-bold text-gray-900 dark:text-white">Synchronization</h2>
            </div>
            <div class="space-y-6">
              <div class="flex items-center justify-between p-4 glass-card-inner rounded-xl">
                <span class="font-medium text-gray-900 dark:text-white">Auto Sync</span>
                <button @click="toggleAutoSync" class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2" :class="autoSyncEnabled ? 'bg-purple-600' : 'bg-gray-700 dark:bg-gray-600'">
                  <span class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform" :class="autoSyncEnabled ? 'translate-x-6' : 'translate-x-1'"></span>
                </button>
              </div>
              <div v-if="autoSyncEnabled" class="glass-card-inner p-4 rounded-xl space-y-3 text-sm">
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Last sync:</span><span class="font-mono">{{ lastSyncTime || 'Never' }}</span></div>
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Next sync:</span><span class="font-mono">{{ nextSyncTime || 'Calculating...' }}</span></div>
                <div class="flex items-center justify-between"><span class="text-gray-600 dark:text-gray-300">Interval:</span><span class="font-medium text-purple-600 dark:text-purple-400">30 minutes</span></div>
              </div>
              <button @click="manualSync" :disabled="loading" class="glass-button-secondary w-full px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100 flex items-center justify-center">
                  <svg v-if="loading" class="animate-spin -ml-1 mr-3 h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                  <span v-if="loading">Syncing...</span>
                  <span v-else>Sync Now</span>
              </button>
            </div>
          </div>

          <div class="glass-card p-6 sm:p-8 rounded-2xl">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Account Actions</h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <button @click="openDashboard" class="glass-button px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20"><path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z"></path><path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z"></path></svg>
                  Open Web Dashboard
              </button>
              <button @click="downloadCredentials" class="glass-button-secondary px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg>
                  Download Credentials
              </button>
              <button @click="refreshFanslyData" :disabled="loading" class="glass-button-secondary px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center">
                  <svg v-if="loading" class="animate-spin -ml-1 mr-2 h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                  <svg v-else class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zM10 18a7.002 7.002 0 006.39-3.992 1 1 0 11-1.885-.666A5.002 5.002 0 015.999 13H9a1 1 0 110-2H4a1 1 0 01-1-1v-4a1 1 0 112 0v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path></svg>
                  Refresh Profile
              </button>
              <button @click="clearAllData" class="bg-red-500/10 text-red-500 hover:text-red-600 dark:text-red-400 hover:bg-red-500/20 dark:hover:text-red-300 px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center">
                  <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z" clip-rule="evenodd"></path></svg>
                  Log Out & Clear Data
              </button>
            </div>
          </div>
        </div>

        <!-- Global Error Display -->
        <div v-if="error" class="glass-card-error p-6 rounded-2xl animate-fade-in mt-8">
          <div class="flex items-start space-x-3">
            <svg class="w-6 h-6 text-red-500 dark:text-red-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
            </svg>
            <div>
              <p class="font-medium text-red-800 dark:text-red-200">Error</p>
              <p class="text-sm mt-1 text-red-700 dark:text-red-300">{{ error }}</p>
            </div>
            <button @click="error = ''" class="ml-auto text-red-600 hover:text-red-800 dark:text-red-400 dark:hover:text-red-300 transition-colors">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
            </svg>
            </button>
          </div>
        </div>

        <!-- Global Success Toast -->
        <div v-if="showSuccessToast" class="fixed top-4 right-4 glass-card p-4 rounded-xl border-green-500/20 bg-green-500/5 animate-slide-in-right z-50">
          <div class="flex items-center space-x-3">
            <svg class="w-6 h-6 text-green-500 dark:text-green-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
            </svg>
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
import { open } from '@tauri-apps/plugin-shell'

let unlistenSyncNow = null
let unlistenToggleAutoSync = null

// Reactive state
const viewState = ref('loading') // 'loading', 'welcome', 'dashboard'
const darkMode = ref(true)
const authToken = ref('')
const showToken = ref(false)
const showPassword = ref(false)
const fanslyData = ref(null)
const accountData = ref(null)
// const isCreator = ref(true) // Removed - now dynamically determined
const loading = ref(false)
const error = ref('')
const autoSyncEnabled = ref(false)
const lastSyncTime = ref('')
const nextSyncTime = ref('')
const syncInterval = ref(null)
const showSuccessToast = ref(false)
const successMessage = ref('')
const subscriptionTiers = ref([])
const followersCount = ref(0)
const subscribersCount = ref(0)
const lastFollowersSync = ref('')
const lastSubscribersSync = ref('')

// Methods
function toggleDarkMode() {
  darkMode.value = !darkMode.value
  localStorage.setItem('darkMode', darkMode.value.toString())
}

function showSuccess(message) {
  successMessage.value = message
  showSuccessToast.value = true
  setTimeout(() => {
    showSuccessToast.value = false
  }, 3000)
}

async function copyToClipboard(text) {
  try {
    await navigator.clipboard.writeText(text)
    showSuccess('Copied to clipboard!')
  } catch (err) {
    console.error('Failed to copy: ', err)
  }
}

async function fetchFanslyData() {
  if (!authToken.value) {
    error.value = 'Please enter your authorization token'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    const data = await invoke('fetch_fansly_data', {
      authToken: authToken.value
    })
    
    fanslyData.value = data
    
    // Save fanslyData to localStorage
    localStorage.setItem('fansly_data', JSON.stringify(data))
    
    showSuccess('Fansly data fetched successfully!')
  } catch (err) {
    error.value = `Failed to fetch Fansly data: ${err}`
  } finally {
    loading.value = false
  }
}

async function fetchSubscriptionTiers() {
  if (!authToken.value || !fanslyData.value) {
    error.value = 'Please fetch your Fansly data first'
    return
  }
  loading.value = true
  error.value = ''
  try {
    const tiers = await invoke('fetch_subscription_tiers', {
      authToken: authToken.value,
      userId: fanslyData.value.id
    })
    subscriptionTiers.value = tiers
    showSuccess('Subscription tiers fetched successfully!')
          
    // This sync call should be handled by sync_all_data now if it's part of the main sync flow
    // if (accountData.value) {
    //   await invoke('sync_data_enhanced', {
    //     syncKey: accountData.value.sync_key,
    //     dataType: 'subscription_tiers',
    //     data: tiers
    //   })
    // }
  } catch (err) {
    error.value = `Failed to fetch subscription tiers: ${err}`
  } finally {
    loading.value = false
  }
}

async function createAccount() {
  if (!fanslyData.value) {
    error.value = 'Please fetch your Fansly data first'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    // Determine if the user is a creator based on subscriber count
    const isUserCreator = (fanslyData.value.subscriberCount && fanslyData.value.subscriberCount > 0);

    if (!isUserCreator) {
        error.value = 'You need to have at least one subscriber on Fansly to create a creator account in the dashboard. If you are not a creator, this dashboard might not be for you.'
        loading.value = false
        return;
    }
    
    const data = await invoke('create_account', {
      fanslyProfile: fanslyData.value 
    })
    
    accountData.value = data
    localStorage.setItem('fansly_account_data', JSON.stringify(data))
    localStorage.setItem('fansly_auth_token', authToken.value)
    localStorage.setItem('fansly_data', JSON.stringify(fanslyData.value))
    
    showSuccess('Account created successfully!')

    viewState.value = 'dashboard';
  } catch (err) {
    // The error from the server will now be more specific!
    error.value = `Failed to create account: ${err}`
  } finally {
    loading.value = false
  }
}


function downloadCredentials() {
  if (!accountData.value) return
  const credentials = {
    account_id: accountData.value.account_id, // Ensure account_id is included
    password: accountData.value.password,     // Ensure password is included
    sync_key: accountData.value.sync_key,
    api_base_url: accountData.value.api_base_url,
    created_at: new Date().toISOString(),
    instructions: {
      sync: 'The sync_key is used for automatic data synchronization',
      security: 'Keep these credentials secure and do not share them'
    }
  }
  const blob = new Blob([JSON.stringify(credentials, null, 2)], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `fansly-credentials-${Date.now()}.json`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
  
  showSuccess('Credentials downloaded!')
}

async function openDashboard() {
  await open('https://creator.notifansly.xyz')
}

function toggleAutoSync() {
  autoSyncEnabled.value = !autoSyncEnabled.value
  localStorage.setItem('auto_sync_enabled', autoSyncEnabled.value.toString())
  
  if (autoSyncEnabled.value) {
    startAutoSync()
    showSuccess('Auto sync enabled!')
  } else {
    stopAutoSync()
    showSuccess('Auto sync disabled!')
  }
}

function startAutoSync() {
  if (syncInterval.value) {
    clearInterval(syncInterval.value)
  }
  
  // Sync every 30 minutes
  syncInterval.value = setInterval(async () => {
    await performSync()
  }, 30 * 60 * 1000)
  
  updateNextSyncTime()
}

function stopAutoSync() {
  if (syncInterval.value) {
    clearInterval(syncInterval.value)
    syncInterval.value = null
  }
  nextSyncTime.value = ''
}

function updateNextSyncTime() {
  if (autoSyncEnabled.value) {
    const next = new Date(Date.now() + 30 * 60 * 1000)
    nextSyncTime.value = next.toLocaleTimeString()
  }
}

async function fetchFollowersAndSubscribers() {
  if (!authToken.value || !fanslyData.value) {
    error.value = 'Please fetch your Fansly data first'
    return
  }
  
  loading.value = true
  error.value = ''
  
  try {
    const data = await invoke('fetch_followers_and_subscribers', {
      authToken: authToken.value,
      userId: fanslyData.value.id
    })
    
    // Process the data as needed
    showSuccess('Followers and subscribers fetched successfully!')
    return data
  } catch (err) {
    error.value = `Failed to fetch followers and subscribers: ${err}`
    console.error('Error fetching followers/subscribers:', err)
  } finally {
    loading.value = false
  }
}

async function performSync() {
  console.log('performSync called')
  console.log('accountData:', accountData.value)
  console.log('authToken:', authToken.value ? 'Present' : 'Missing')
  console.log('fanslyData:', fanslyData.value)
      
  if (!accountData.value) {
    error.value = 'Account data is missing. Please create an account first.'
    return
  }
  if (!authToken.value) {
    error.value = 'Authorization token is missing. Please enter your Fansly auth token.'
    return
  }
  if (!fanslyData.value) {
    error.value = 'Fansly data is missing. Please fetch your Fansly data first.'
    return
  }
  if (!accountData.value.sync_key) {
    error.value = 'Sync key is missing from account data.'
    return
  }
      
  loading.value = true
  error.value = ''
      
  try {
    console.log('Starting sync with:', {
      syncKey: accountData.value.sync_key,
      hasAuthToken: !!authToken.value
    })
          
    // Sync all data
    const syncResult = await invoke('sync_all_data', {
      syncKey: accountData.value.sync_key,
      authToken: authToken.value,
      userId: fanslyData.value.id
    })
          
    lastSyncTime.value = new Date().toLocaleTimeString()
    updateNextSyncTime()
          
    showSuccess(`Sync completed! ${syncResult.message || 'Success'}`)
  } catch (err) {
    error.value = `Sync failed: ${err}`
    console.error('Sync error:', err)
  } finally {
    loading.value = false
  }
}

async function manualSync() {
  await manualSyncWithProgress()
}

async function manualSyncWithProgress() {
  if (!accountData.value) {
    error.value = 'Account data is missing. Please create an account first.'
    return
  }
  if (!authToken.value) {
    error.value = 'Authorization token is missing. Please enter your Fansly auth token.'
    return
  }
      
  loading.value = true
  error.value = ''
      
  try {
    // Auto-fetch fansly data if missing
    if (!fanslyData.value) {
      showSuccess('Fetching your Fansly profile data...')
      const data = await invoke('fetch_fansly_data', {
        authToken: authToken.value
      })
      fanslyData.value = data
      localStorage.setItem('fansly_data', JSON.stringify(data))
    }
          
    // Sync all data (this backend call should fetch followers, subscribers, and tiers)
    showSuccess('Syncing all data to server...')
    const syncResult = await invoke('sync_all_data', {
      syncKey: accountData.value.sync_key,
      authToken: authToken.value,
      userId: fanslyData.value.id
    })
          
    lastSyncTime.value = new Date().toLocaleTimeString()
    updateNextSyncTime()
          
    showSuccess(`Manual sync completed! ${syncResult.length ? `${syncResult.length} items synced!` : 'Success'}`)
  } catch (err) {
    error.value = `Sync failed: ${err}`
    console.error('Sync error details:', err)
  } finally {
    loading.value = false
  }
}

async function testSyncWithEmptyData() {
  if (!accountData.value) {
    error.value = 'Account data is missing.'
    return
  }
  loading.value = true
  error.value = ''
  try {
    await invoke('sync_data_enhanced', {
      syncKey: accountData.value.sync_key,
      dataType: 'test',
      data: {
        followers: [],
        subscribers: [],
        subscription_tiers: [],
        test_mode: true,
        timestamp: new Date().toISOString()
      }
    })
    showSuccess('Test sync completed successfully!')
  } catch (err) {
    error.value = `Test sync failed: ${err}`
  } finally {
    loading.value = false
  }
}

function debugCurrentState() {
  console.log('=== DEBUG STATE ===')
  console.log('accountData:', accountData.value)
  console.log('authToken length:', authToken.value?.length || 0)
  console.log('fanslyData:', fanslyData.value)
  console.log('autoSyncEnabled:', autoSyncEnabled.value)
  console.log('==================')
}

function clearAllData() {
  localStorage.removeItem('fansly_account_data')
  localStorage.removeItem('fansly_auth_token')
  localStorage.removeItem('fansly_data')
  localStorage.removeItem('auto_sync_enabled')
  
  accountData.value = null
  authToken.value = ''
  fanslyData.value = null
  autoSyncEnabled.value = false
  
  viewState.value = 'welcome';
  showSuccess('All data cleared!')
}

async function refreshFanslyData() {
  if (!authToken.value) {
    error.value = 'Auth token is missing. Please enter it again.'
    return
  }
  
  await fetchFanslyData()
}

// Lifecycle - FIXED: Made the callback async
onMounted(async () => {
  const savedAccountData = localStorage.getItem('fansly_account_data');
  const savedAuthToken = localStorage.getItem('fansly_auth_token');
  const savedFanslyData = localStorage.getItem('fansly_data');
  const savedAutoSync = localStorage.getItem('auto_sync_enabled');
  const savedDarkMode = localStorage.getItem('darkMode');

  if (savedAccountData && savedAuthToken) {
    try {
      accountData.value = JSON.parse(savedAccountData);
      authToken.value = savedAuthToken;
      if (savedFanslyData) fanslyData.value = JSON.parse(savedFanslyData);

      // We have everything, go to dashboard
      viewState.value = 'dashboard';
    } catch (e) {
      console.error('Failed to parse saved data, clearing...');
      clearAllData();
      viewState.value = 'welcome';
    }
  } else {
    // Not fully set up, start at the beginning
    viewState.value = 'welcome';
  }
  
  // Restore other settings
  if (savedAutoSync === 'true' && viewState.value === 'dashboard') {
    autoSyncEnabled.value = true;
    startAutoSync();
  }
  if (savedDarkMode !== null) {
    darkMode.value = savedDarkMode === 'true';
  }
  
  // Listen for tray events - NOW PROPERLY AWAITED
  try {
    unlistenSyncNow = await listen('tray-sync-now', async () => {
      console.log('Sync triggered from system tray')
      await manualSyncWithProgress()
    })

    unlistenToggleAutoSync = await listen('tray-toggle-auto-sync', async () => {
      console.log('Auto sync toggled from system tray')
      toggleAutoSync()
    })
  } catch (err) {
    console.error('Failed to set up tray event listeners:', err)
  }
  
  // Debug current state
  debugCurrentState()
})

onUnmounted(() => {
  // Clean up event listeners
  if (unlistenSyncNow) unlistenSyncNow()
  if (unlistenToggleAutoSync) unlistenToggleAutoSync()
  
  // Clean up sync interval
  if (syncInterval.value) {
    clearInterval(syncInterval.value)
  }
})
</script>

<style>
/* Glass morphism styles with light/dark mode variants */
.glass-card {
  @apply backdrop-blur-xl shadow-2xl;
  @apply bg-white/50 border-black/5;
  @apply dark:bg-white/10 dark:border-white/20;
}

.glass-card-inner {
  @apply backdrop-blur-lg;
  @apply bg-black/5 border-black/5;
  @apply dark:bg-white/5 dark:border-white/10;
}

.glass-card-error {
  @apply backdrop-blur-lg;
  @apply border-red-500/20 bg-red-500/5;
}

.glass-input {
  @apply backdrop-blur-lg placeholder-gray-500 dark:placeholder-gray-400;
  @apply text-gray-900 bg-white/60 border-black/10;
  @apply dark:text-white dark:bg-white/10 dark:border-white/20;
}

.glass-input:focus {
  @apply bg-white/70 border-black/20;
  @apply dark:bg-white/15 dark:border-white/30 outline-none;
}

.glass-button {
  @apply backdrop-blur-lg bg-gradient-to-r from-purple-600 to-pink-600 border-transparent text-white shadow-lg;
}

.glass-button:hover {
  @apply from-purple-500 to-pink-500 shadow-xl;
}


.glass-button-secondary {
  @apply backdrop-blur-lg shadow-lg;
  @apply bg-black/5 border-black/10 text-gray-800; /* Add light mode text color */
  @apply dark:bg-white/10 dark:border-white/20 dark:text-white; /* Add dark mode text color */
}

.glass-button-secondary:hover {
  @apply bg-black/10 shadow-xl;
  @apply dark:bg-white/15;
}


.glass-code {
  @apply backdrop-blur-lg border;
  /* This ensures the code block text is dark in light-mode */
  @apply bg-black/5 border-black/10 text-gray-800;
  @apply dark:bg-black/20 dark:border-white/10 dark:text-gray-200;
}

.noise-bg {
  background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 1024 1024' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E");
}

/* Animations */
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slide-in-right {
  from {
    opacity: 0;
    transform: translateX(100%);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.6s ease-out;
}

.animate-slide-in-right {
  animation: slide-in-right 0.3s ease-out;
}

/* Dark mode utilities */
.dark {
  color-scheme: dark;
}

/* Custom scrollbar with light/dark mode */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
}

.dark ::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
}

::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.dark ::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.dark ::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
}
</style>
