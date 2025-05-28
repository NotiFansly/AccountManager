<template>
  <div class="min-h-screen transition-all duration-500" :class="darkMode ? 'dark' : ''">
    <!-- Background with gradient -->
    <div class="fixed inset-0 bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 dark:from-gray-900 dark:via-purple-900 dark:to-indigo-900"></div>
    
    <!-- Animated background elements -->
    <div class="fixed inset-0 overflow-hidden pointer-events-none">
      <div class="absolute -top-40 -right-40 w-80 h-80 bg-purple-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob"></div>
      <div class="absolute -bottom-40 -left-40 w-80 h-80 bg-blue-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000"></div>
      <div class="absolute top-40 left-40 w-80 h-80 bg-pink-500 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000"></div>
    </div>

    <div class="relative z-10 min-h-screen p-4 sm:p-8">
      <!-- Header with dark mode toggle -->
      <header class="flex justify-between items-center mb-8">
        <h1 class="text-4xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
          Fansly Account Manager
        </h1>
        <button
          @click="toggleDarkMode"
          class="glass-card p-3 rounded-xl hover:scale-105 transition-all duration-300"
        >
          <svg v-if="darkMode" class="w-6 h-6 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clip-rule="evenodd"></path>
          </svg>
          <svg v-else class="w-6 h-6 text-purple-400" fill="currentColor" viewBox="0 0 20 20">
            <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z"></path>
          </svg>
        </button>
      </header>

      <div class="max-w-6xl mx-auto space-y-8">
        <!-- Step 1: Fetch Fansly Data -->
        <div class="glass-card p-8 rounded-2xl hover:scale-[1.02] transition-all duration-300">
          <div class="flex items-center mb-6">
            <div class="w-10 h-10 bg-gradient-to-r from-purple-500 to-pink-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
              1
            </div>
            <h2 class="text-2xl font-bold text-white">Fetch Your Fansly Data</h2>
          </div>
          
          <div class="space-y-6">
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-3">Authorization Token</label>
              <div class="relative">
                <input
                  v-model="authToken"
                  :type="showToken ? 'text' : 'password'"
                  placeholder="Enter your Fansly auth token"
                  class="glass-input w-full px-4 py-3 rounded-xl focus:ring-2 focus:ring-purple-500 transition-all duration-300"
                />
                <button
                  @click="showToken = !showToken"
                  class="absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 hover:text-white transition-colors"
                >
                  <svg v-if="showToken" class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd"></path>
                    <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z"></path>
                  </svg>
                  <svg v-else class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                    <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                  </svg>
                </button>
              </div>
            </div>
            
            <button
              @click="fetchFanslyData"
              :disabled="!authToken || loading"
              class="glass-button px-8 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <div class="flex items-center justify-center">
                <svg v-if="loading" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                {{ loading ? 'Fetching...' : 'Fetch Data' }}
              </div>
            </button>
          </div>
        </div>

        <!-- Step 2: Review Data -->
        <div v-if="fanslyData" class="glass-card p-8 rounded-2xl hover:scale-[1.02] transition-all duration-300 animate-fade-in">
          <div class="flex items-center mb-6">
            <div class="w-10 h-10 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
              2
            </div>
            <h2 class="text-2xl font-bold text-white">Review Your Data</h2>
          </div>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-purple-400 rounded-full"></div>
                <span class="text-gray-300">Username:</span>
                <span class="text-white font-semibold">{{ fanslyData.username }}</span>
              </div>
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-blue-400 rounded-full"></div>
                <span class="text-gray-300">Display Name:</span>
                <span class="text-white font-semibold">{{ fanslyData.displayName }}</span>
              </div>
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-pink-400 rounded-full"></div>
                <span class="text-gray-300">Email:</span>
                <span class="text-white font-semibold">{{ fanslyData.email }}</span>
              </div>
            </div>
            <div class="space-y-4">
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                <span class="text-gray-300">User ID:</span>
                <span class="text-white font-semibold font-mono text-sm">{{ fanslyData.id }}</span>
              </div>
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-yellow-400 rounded-full"></div>
                <span class="text-gray-300">Followers:</span>
                <span class="text-white font-semibold">{{ fanslyData.followCount?.toLocaleString() || 0 }}</span>
              </div>
              <div class="flex items-center space-x-3">
                <div class="w-2 h-2 bg-red-400 rounded-full"></div>
                <span class="text-gray-300">Subscribers:</span>
                <span class="text-white font-semibold">{{ fanslyData.subscriberCount?.toLocaleString() || 0 }}</span>
              </div>
            </div>
          </div>

          <!-- Add refresh button -->
          <button
            @click="refreshFanslyData"
            :disabled="loading"
            class="glass-button-secondary px-4 py-2 rounded-lg font-medium transition-all duration-300 hover:scale-105 disabled:opacity-50"
          >
            <div class="flex items-center">
              <svg v-if="loading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <svg v-else class="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
              </svg>
              {{ loading ? 'Refreshing...' : 'Refresh' }}
            </div>
          </button>
        </div>

        <!-- Step 3: Create Account -->
        <div v-if="fanslyData" class="glass-card p-8 rounded-2xl hover:scale-[1.02] transition-all duration-300 animate-fade-in">
          <div class="flex items-center mb-6">
            <div class="w-10 h-10 bg-gradient-to-r from-green-500 to-emerald-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
              3
            </div>
            <h2 class="text-2xl font-bold text-white">Create Dashboard Account</h2>
          </div>
          
          <div class="space-y-6">
            <div class="flex items-center space-x-3">
              <input
                v-model="isCreator"
                type="checkbox"
                id="isCreator"
                class="w-5 h-5 text-purple-600 bg-transparent border-2 border-gray-400 rounded focus:ring-purple-500 focus:ring-2"
              />
              <label for="isCreator" class="text-white font-medium cursor-pointer">I am a content creator</label>
            </div>
            
            <button
              @click="createAccount"
              :disabled="loading"
              class="glass-button px-8 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <div class="flex items-center justify-center">
                <svg v-if="loading" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                {{ loading ? 'Creating...' : 'Create Account' }}
              </div>
            </button>
          </div>
        </div>

        <!-- Step 4: Account Created -->
        <div v-if="accountData" class="glass-card p-8 rounded-2xl hover:scale-[1.02] transition-all duration-300 animate-fade-in">
          <div class="flex items-center mb-6">
            <div class="w-10 h-10 bg-gradient-to-r from-emerald-500 to-teal-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
              ✓
            </div>
            <h2 class="text-2xl font-bold text-white">Account Created Successfully!</h2>
          </div>
          
          <div class="space-y-6">
            <div class="glass-card-inner p-6 rounded-xl border border-white/10">
              <div class="space-y-4">
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
                  <span class="text-gray-300 font-medium">Account ID:</span>
                  <div class="flex items-center space-x-2">
                    <code class="glass-code px-3 py-1 rounded-lg text-sm font-mono">{{ accountData.account_id }}</code>
                    <button @click="copyToClipboard(accountData.account_id)" class="text-purple-400 hover:text-purple-300 transition-colors">
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path>
                        <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                
                <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
                  <span class="text-gray-300 font-medium">Password:</span>
                  <div class="flex items-center space-x-2">
                    <code class="glass-code px-3 py-1 rounded-lg text-sm font-mono">{{ showPassword ? accountData.password : '••••••••' }}</code>
                    <button @click="showPassword = !showPassword" class="text-purple-400 hover:text-purple-300 transition-colors">
                      <svg v-if="showPassword" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd"></path>
                        <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z"></path>
                      </svg>
                      <svg v-else class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                        <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                      </svg>
                    </button>
                    <button @click="copyToClipboard(accountData.password)" class="text-purple-400 hover:text-purple-300 transition-colors">
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path>
                        <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path>
                      </svg>
                    </button>
                  </div>
                </div>
                
                <div class="flex flex-col">
                  <span class="text-gray-300 font-medium mb-2">Sync Key:</span>
                  <div class="flex items-center space-x-2">
                    <code class="glass-code px-3 py-1 rounded-lg text-xs font-mono break-all flex-1">{{ accountData.sync_key }}</code>
                    <button @click="copyToClipboard(accountData.sync_key)" class="text-purple-400 hover:text-purple-300 transition-colors flex-shrink-0">
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M8 3a1 1 0 011-1h2a1 1 0 110 2H9a1 1 0 01-1-1z"></path>
                        <path d="M6 3a2 2 0 00-2 2v11a2 2 0 002 2h8a2 2 0 002-2V5a2 2 0 00-2-2 3 3 0 01-3 3H9a3 3 0 01-3-3z"></path>
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="glass-card-warning p-4 rounded-xl border border-yellow-500/20 bg-yellow-500/5">
              <div class="flex items-start space-x-3">
                <svg class="w-6 h-6 text-yellow-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
                </svg>
                <div>
                  <p class="text-yellow-200 font-medium">Important!</p>
                  <p class="text-yellow-300 text-sm mt-1">Save these credentials securely. You'll need them to log into the dashboard.</p>
                </div>
              </div>
            </div>
            
            <div class="flex flex-col sm:flex-row gap-4">
              <button
                @click="downloadCredentials"
                class="glass-button-secondary px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center"
              >
                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                </svg>
                Download Credentials
              </button>
              
              <button
                @click="openDashboard"
                class="glass-button px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 flex items-center justify-center"
              >
                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z"></path>
                  <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z"></path>
                </svg>
                Open Dashboard
              </button>
            </div>
          </div>
        </div>

        <!-- Step 5: Auto Sync -->
        <div v-if="accountData" class="glass-card p-8 rounded-2xl hover:scale-[1.02] transition-all duration-300 animate-fade-in">
          <div class="flex items-center mb-6">
            <div class="w-10 h-10 bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full flex items-center justify-center text-white font-bold mr-4">
              5
            </div>
            <h2 class="text-2xl font-bold text-white">Enable Auto Sync</h2>
          </div>
          
          <div class="space-y-6">
            <div class="flex items-center justify-between p-4 glass-card-inner rounded-xl">
              <div class="flex items-center space-x-3">
                <svg class="w-6 h-6 text-purple-400" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                </svg>
                <span class="text-white font-medium">Auto sync followers and subscribers</span>
              </div>
              
              <button
                @click="toggleAutoSync"
                class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2"
                :class="autoSyncEnabled ? 'bg-purple-600' : 'bg-gray-600'"
              >
                <span
                  class="inline-block h-4 w-4 transform rounded-full bg-white transition-transform"
                  :class="autoSyncEnabled ? 'translate-x-6' : 'translate-x-1'"
                ></span>
              </button>
            </div>
            
            <div v-if="autoSyncEnabled" class="glass-card-inner p-4 rounded-xl space-y-3">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-300">Last sync:</span>
                <span class="text-white font-mono">{{ lastSyncTime || 'Never' }}</span>
              </div>
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-300">Next sync:</span>
                <span class="text-white font-mono">{{ nextSyncTime || 'Calculating...' }}</span>
              </div>
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-300">Sync interval:</span>
                <span class="text-purple-400 font-medium">Every 30 minutes</span>
              </div>
            </div>
            
            <button
              @click="manualSync"
              :disabled="!autoSyncEnabled || loading"
              class="glass-button-secondary w-full px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
            >
              <div class="flex items-center justify-center">
                <svg v-if="loading" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                <svg v-else class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                  <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
                </svg>
                {{ loading ? 'Syncing...' : 'Manual Sync Now' }}
              </div>
            </button>

            <!-- Test sync button for debugging 
      <button
        @click="testSyncWithEmptyData"
        :disabled="loading"
        class="glass-button-secondary px-6 py-3 rounded-xl font-semibold transition-all duration-300 hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100"
      >
        <div class="flex items-center justify-center">
          <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
          </svg>
          Test Sync
        </div>
</button>-->
          </div>
        </div>

        <!-- Error Display -->
        <div v-if="error" class="glass-card-error p-6 rounded-2xl border border-red-500/20 bg-red-500/5 animate-fade-in">
          <div class="flex items-start space-x-3">
            <svg class="w-6 h-6 text-red-400 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
            </svg>
            <div>
              <p class="text-red-200 font-medium">Error</p>
              <p class="text-red-300 text-sm mt-1">{{ error }}</p>
            </div>
            <button @click="error = ''" class="ml-auto text-red-400 hover:text-red-300 transition-colors">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
              </svg>
            </button>
          </div>
        </div>

        <!-- Success Toast -->
        <div v-if="showSuccessToast" class="fixed top-4 right-4 glass-card p-4 rounded-xl border border-green-500/20 bg-green-500/5 animate-slide-in-right z-50">
          <div class="flex items-center space-x-3">
            <svg class="w-6 h-6 text-green-400" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
            </svg>
            <span class="text-green-200 font-medium">{{ successMessage }}</span>
          </div>
        </div>
      </div>
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
const darkMode = ref(true)
const authToken = ref('')
const showToken = ref(false)
const showPassword = ref(false)
const fanslyData = ref(null)
const accountData = ref(null)
const isCreator = ref(true)
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
    
    // Also sync the tiers data
    if (accountData.value) {
      await invoke('sync_data_enhanced', {
        sync_key: accountData.value.sync_key,
        data_type: 'tiers',
        data: tiers
      })
    }
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
    // Extract the necessary data from fanslyData
    const userData = fanslyData.value;
    
    // Make sure we have the required data
    if (!userData || !userData.id) {
      error.value = 'Missing required user data. Please fetch your Fansly data again.'
      loading.value = false
      return
    }
    
    const data = await invoke('create_account', {
      fanslyUserId: userData.id,
      email: userData.email || '',
      username: userData.username || '',
      displayName: userData.display_name || '',
      isCreator: isCreator.value
    })
    
    accountData.value = data
    localStorage.setItem('fansly_account_data', JSON.stringify(data))
    localStorage.setItem('fansly_auth_token', authToken.value)
    // Also save fanslyData when creating account
    localStorage.setItem('fansly_data', JSON.stringify(fanslyData.value))
    
    showSuccess('Account created successfully!')
  } catch (err) {
    error.value = `Failed to create account: ${err}`
  } finally {
    loading.value = false
  }
}


function downloadCredentials() {
  if (!accountData.value) return
  const credentials = {
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
  await open('https://your-dashboard-url.com')
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
        
    // Fetch followers and subscribers
    showSuccess('Fetching followers and subscribers...')
    let syncData
    try {
      syncData = await invoke('fetch_followers_and_subscribers', {
        authToken: authToken.value,
        userId: fanslyData.value.id
      })
      console.log('Successfully fetched followers/subscribers:', syncData)
    } catch (err) {
      console.error('Error fetching followers/subscribers:', err)
      error.value = `Failed to fetch followers/subscribers: ${err}`
      return
    }
        
    // Fetch subscription tiers
    showSuccess('Fetching subscription tiers...')
    let tiers
    try {
      tiers = await invoke('fetch_subscription_tiers', {
        authToken: authToken.value,
        userId: fanslyData.value.id
      })
      console.log('Successfully fetched subscription tiers:', tiers)
    } catch (err) {
      console.error('Error fetching subscription tiers:', err)
      error.value = `Failed to fetch subscription tiers: ${err}`
      return
    }
        
    // Sync all data
    showSuccess('Syncing data to server...')
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
  // Load saved data from localStorage
  const savedAccountData = localStorage.getItem('fansly_account_data')
  if (savedAccountData) {
    try {
      accountData.value = JSON.parse(savedAccountData)
      console.log('Loaded account data from localStorage:', accountData.value)
    } catch (e) {
      console.error('Failed to parse saved account data:', e)
      localStorage.removeItem('fansly_account_data')
    }
  }
  
  const savedAuthToken = localStorage.getItem('fansly_auth_token')
  if (savedAuthToken) {
    authToken.value = savedAuthToken
    console.log('Loaded auth token from localStorage')
  }
  
  // Load fanslyData from localStorage
  const savedFanslyData = localStorage.getItem('fansly_data')
  if (savedFanslyData) {
    try {
      fanslyData.value = JSON.parse(savedFanslyData)
      console.log('Loaded fansly data from localStorage:', fanslyData.value)
    } catch (e) {
      console.error('Failed to parse saved fansly data:', e)
      localStorage.removeItem('fansly_data')
    }
  }
  
  const savedAutoSync = localStorage.getItem('auto_sync_enabled')
  if (savedAutoSync === 'true') {
    autoSyncEnabled.value = true
    startAutoSync()
  }
  
  const savedDarkMode = localStorage.getItem('darkMode')
  if (savedDarkMode !== null) {
    darkMode.value = savedDarkMode === 'true'
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
/* Glass morphism styles */
.glass-card {
  @apply backdrop-blur-xl bg-white/10 border border-white/20 shadow-2xl;
}

.glass-card-inner {
  @apply backdrop-blur-lg bg-white/5 border border-white/10;
}

.glass-card-warning {
  @apply backdrop-blur-lg;
}

.glass-card-error {
  @apply backdrop-blur-lg;
}

.glass-input {
  @apply backdrop-blur-lg bg-white/10 border border-white/20 text-white placeholder-gray-400;
}

.glass-input:focus {
  @apply bg-white/15 border-white/30 outline-none;
}

.glass-button {
  @apply backdrop-blur-lg bg-gradient-to-r from-purple-600/80 to-pink-600/80 border border-white/20 text-white shadow-lg;
}

.glass-button:hover {
  @apply from-purple-500/90 to-pink-500/90 shadow-xl;
}

.glass-button-secondary {
  @apply backdrop-blur-lg bg-white/10 border border-white/20 text-white shadow-lg;
}

.glass-button-secondary:hover {
  @apply bg-white/15 shadow-xl;
}

.glass-code {
  @apply backdrop-blur-lg bg-black/20 border border-white/10 text-gray-200;
}

/* Animations */
@keyframes blob {
  0% {
    transform: translate(0px, 0px) scale(1);
  }
  33% {
    transform: translate(30px, -50px) scale(1.1);
  }
  66% {
    transform: translate(-20px, 20px) scale(0.9);
  }
  100% {
    transform: translate(0px, 0px) scale(1);
  }
}

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

.animate-blob {
  animation: blob 7s infinite;
}

.animation-delay-2000 {
  animation-delay: 2s;
}

.animation-delay-4000 {
  animation-delay: 4s;
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

/* Custom scrollbar */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.3);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.5);
}
</style>

