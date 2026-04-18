const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

// UI Elements
const timerDisplay = document.getElementById('timer-display');
const statusText = document.getElementById('status-text');

const btnStart = document.getElementById('btn-start');
const btnPause = document.getElementById('btn-pause');
const btnReset = document.getElementById('btn-reset');
const btnSaveSettings = document.getElementById('btn-save-settings');

const modeWork = document.getElementById('mode-work');
const modeShortBreak = document.getElementById('mode-short-break');
const modeLongBreak = document.getElementById('mode-long-break');

const inputWork = document.getElementById('input-work');
const inputShortBreak = document.getElementById('input-short-break');
const inputLongBreak = document.getElementById('input-long-break');

// State
let isRunning = false;
let currentMode = 'work';

// Format seconds to MM:SS
function formatTime(seconds) {
  const m = Math.floor(seconds / 60).toString().padStart(2, '0');
  const s = (seconds % 60).toString().padStart(2, '0');
  return `${m}:${s}`;
}

// Update UI Mode styling
function updateUIForMode(mode) {
  document.body.className = `mode-${mode.replace('_', '-')}`;

  [modeWork, modeShortBreak, modeLongBreak].forEach(btn => btn.classList.remove('active'));

  if (mode === 'work') {
    modeWork.classList.add('active');
    statusText.textContent = 'Focus Time';
  } else if (mode === 'short_break') {
    modeShortBreak.classList.add('active');
    statusText.textContent = 'Short Break';
  } else if (mode === 'long_break') {
    modeLongBreak.classList.add('active');
    statusText.textContent = 'Long Break';
  }
}

// Listen for tick events from Rust
listen('timer-tick', (event) => {
  const payload = event.payload; // { seconds_remaining, mode, is_running }

  timerDisplay.textContent = formatTime(payload.seconds_remaining);

  if (payload.mode !== currentMode) {
    currentMode = payload.mode;
    updateUIForMode(payload.mode);
  }

  if (payload.is_running !== isRunning) {
    isRunning = payload.is_running;
    if (isRunning) {
      btnStart.classList.add('hidden');
      btnPause.classList.remove('hidden');
    } else {
      btnStart.classList.remove('hidden');
      btnPause.classList.add('hidden');
    }
  }
});

// Setup Commands
btnStart.addEventListener('click', () => invoke('start_timer'));
btnPause.addEventListener('click', () => invoke('pause_timer'));
btnReset.addEventListener('click', () => invoke('reset_timer'));

btnSaveSettings.addEventListener('click', () => {
  const work = parseInt(inputWork.value, 10);
  const shortBreak = parseInt(inputShortBreak.value, 10);
  const longBreak = parseInt(inputLongBreak.value, 10);

  if (work > 0 && shortBreak > 0 && longBreak > 0) {
    invoke('set_durations', { work, shortBreak, longBreak });
    btnSaveSettings.textContent = "Saved!";
    setTimeout(() => {
        btnSaveSettings.textContent = "Save Settings";
    }, 2000);
  }
});

// Mode manual change
modeWork.addEventListener('click', () => invoke('set_mode', { mode: 'work' }));
modeShortBreak.addEventListener('click', () => invoke('set_mode', { mode: 'short_break' }));
modeLongBreak.addEventListener('click', () => invoke('set_mode', { mode: 'long_break' }));

// Initial state fetch if needed (optional since rust emits immediately when started)
// invoke('get_state');
