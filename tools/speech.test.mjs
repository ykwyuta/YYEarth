import test from 'node:test';
import assert from 'node:assert/strict';
import { createNarrator } from '../web/speech.mjs';
function setup() {
  const played = [];
  let cancelled = 0;
  class Utterance { constructor(text) { this.text = text; } }
  const voices = [{ lang: 'en-US' }, { lang: 'ja-JP' }, { lang: 'zh-TW' }];
  const synth = { cancel() { cancelled++; }, getVoices: () => voices, speak: u => played.push(u) };
  return { narrator: createNarrator(synth, Utterance), played, voices, cancels: () => cancelled };
}
test('waits for interaction and retains only the newest guide', () => {
  const { narrator, played } = setup();
  narrator.say('old scene', 'en-US');
  narrator.say('new scene', 'ja-JP');
  assert.equal(played.length, 0);
  narrator.unlock();
  assert.equal(played.length, 1);
  assert.equal(played[0].text, 'new scene');
  narrator.unlock();
  assert.equal(played.length, 1);
});
test('uses the selected language and a slower speaking rate', () => {
  const { narrator, played, voices } = setup();
  narrator.unlock();
  narrator.say('こんにちは', 'ja-JP');
  assert.equal(played[0].voice, voices[1]);
  assert.equal(played[0].lang, 'ja-JP');
  assert.equal(played[0].rate, 0.85);
  narrator.say('你好', 'zh-CN');
  assert.equal(played[1].voice, voices[2]);
});
test('replay interrupts previous speech without appending stale guides', () => {
  const { narrator, played, cancels } = setup();
  narrator.unlock();
  narrator.say('guide', 'en-US');
  narrator.say('guide', 'en-US');
  assert.equal(played.length, 2);
  assert.equal(cancels(), 2);
});
test('turning off before interaction discards pending speech', () => {
  const { narrator, played } = setup();
  narrator.say('guide', 'ja-JP');
  narrator.say('', 'ja-JP');
  narrator.unlock();
  assert.equal(played.length, 0);
});
test('stop cancels current speech and clears pending content', () => {
  const { narrator, played, cancels } = setup();
  narrator.unlock();
  narrator.say('guide', 'ja-JP');
  narrator.stop();
  narrator.unlock();
  assert.equal(cancels(), 2);
  assert.equal(played.length, 1);
});
test('unsupported speech is harmless', () => {
  const narrator = createNarrator(undefined, undefined);
  assert.equal(narrator.supported, false);
  narrator.say('guide', 'ja-JP');
  narrator.unlock();
  narrator.stop();
});
