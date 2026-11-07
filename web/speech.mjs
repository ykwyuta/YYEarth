// Keep only the latest guide; scene changes and replay interrupt stale speech.
export function createNarrator(synth, Utterance) {
  let unlocked = false;
  let pending = null;
  let current = null; // Keep a reference until completion (also needed by Safari).
  const supported = !!synth && typeof Utterance === 'function';
  function stop() {
    pending = null;
    current = null;
    if (supported) synth.cancel();
  }
  function play() {
    if (!supported || !unlocked || !pending) return;
    const { text, lang } = pending;
    pending = null;
    synth.cancel();
    const utterance = new Utterance(text);
    utterance.lang = lang;
    utterance.rate = 0.85;
    const voices = synth.getVoices();
    utterance.voice = voices.find(v => v.lang.toLowerCase() === lang.toLowerCase())
      || voices.find(v => v.lang.split('-')[0] === lang.split('-')[0]) || null;
    utterance.onend = utterance.onerror = () => {
      if (current === utterance) current = null;
    };
    current = utterance;
    synth.speak(utterance);
  }
  return {
    supported,
    stop,
    unlock() { unlocked = true; play(); },
    say(text, lang) {
      if (!text) { stop(); return; }
      pending = { text, lang };
      play();
    },
  };
}
